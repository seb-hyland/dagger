use crate::trust_me_bro;
use std::{
    any::Any,
    array,
    mem::MaybeUninit,
    ops::{Index, IndexMut, Not},
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    slice,
    sync::mpsc::{self, Sender, SyncSender},
    thread::{self, Scope},
};

pub struct Scheduler<'scope, 'env, const NUM_TASKS: usize> {
    scope: &'scope Scope<'scope, 'env>,
    tasks: Slice<Task<'scope>, NUM_TASKS>,
    completed_tasks: usize,
    threads: ArrayVec<Thread<'scope>, NUM_TASKS>,
}

struct Thread<'scope> {
    sender: Sender<Message<'scope>>,
    busy: bool,
}

pub struct Task<'scope> {
    num_parents: u32,
    completed_parents: u32,
    children: &'scope [usize],
    task: &'scope (dyn Fn() + Send + Sync),
}
impl<'scope> Task<'scope> {
    pub fn new(
        num_parents: u32,
        children: &'scope [usize],
        task: &'scope (dyn Fn() + Send + Sync),
    ) -> Task<'scope> {
        Task {
            num_parents,
            completed_parents: 0,
            children,
            task,
        }
    }
}

enum Message<'scope> {
    Task(TaskMsg<'scope>),
    Shutdown,
}
struct TaskMsg<'scope> {
    task: &'scope (dyn Fn() + Send + Sync),
    thread_id: usize,
    task_id: usize,
}

struct TaskResult {
    thread_id: usize,
    task_id: usize,
    panic: Option<Box<dyn Any + Send + 'static>>,
}

enum Slice<T, const N: usize> {
    Stack([T; N]),
    Heap(Box<[T]>),
}
impl<T, const N: usize> Index<usize> for Slice<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Stack(arr) => &arr[index],
            Self::Heap(arr) => &arr[index],
        }
    }
}
impl<T, const N: usize> IndexMut<usize> for Slice<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Self::Stack(arr) => &mut arr[index],
            Self::Heap(arr) => &mut arr[index],
        }
    }
}
impl<T, const N: usize> Slice<T, N> {
    fn len(&self) -> usize {
        match self {
            Self::Stack(arr) => arr.len(),
            Self::Heap(arr) => arr.len(),
        }
    }
}

enum ArrayVec<T, const N: usize> {
    Array([MaybeUninit<T>; N], usize),
    Vec(Vec<T>),
}
impl<T, const N: usize> ArrayVec<T, N> {
    fn new_array() -> Self {
        Self::Array(array::from_fn(|_| MaybeUninit::uninit()), 0)
    }
    fn new_vec() -> Self {
        Self::Vec(Vec::new())
    }
    fn iter(&self) -> slice::Iter<'_, T> {
        match self {
            Self::Array(arr, len) => {
                let slice = trust_me_bro! {
                    slice::from_raw_parts(arr.as_ptr() as *const T, *len)
                };
                slice.iter()
            }
            Self::Vec(vec) => vec.iter(),
        }
    }
    fn push(&mut self, value: T) {
        match self {
            Self::Array(arr, len) => {
                arr[*len] = MaybeUninit::new(value);
                *len += 1;
            }
            Self::Vec(vec) => vec.push(value),
        }
    }
    fn len(&self) -> usize {
        match self {
            Self::Array(_, len) => *len,
            Self::Vec(vec) => vec.len(),
        }
    }
}
impl<T, const N: usize> Index<usize> for ArrayVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Array(arr, len) => {
                if index >= *len {
                    unreachable!("Index into uninitialized memory!");
                } else {
                    trust_me_bro! { arr[index].assume_init_ref() }
                }
            }
            Self::Vec(vec) => &vec[index],
        }
    }
}
impl<T, const N: usize> IndexMut<usize> for ArrayVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Self::Array(arr, len) => {
                if index >= *len {
                    unreachable!("Index into uninitialized memory!");
                } else {
                    trust_me_bro! { arr[index].assume_init_mut() }
                }
            }
            Self::Vec(vec) => &mut vec[index],
        }
    }
}
impl<T, const N: usize> Drop for ArrayVec<T, N> {
    fn drop(&mut self) {
        match self {
            ArrayVec::Array(arr, len) =>
            {
                #[allow(clippy::needless_range_loop)]
                for i in 0..*len {
                    unsafe {
                        arr[i].assume_init_drop();
                    }
                }
            }
            ArrayVec::Vec(_) => {}
        }
    }
}

impl<'scope, 'env, const NUM_TASKS: usize> Scheduler<'scope, 'env, NUM_TASKS> {
    pub fn execute(tasks: [Task<'scope>; NUM_TASKS]) {
        thread::scope(|s| {
            let scheduler = Scheduler {
                scope: s,
                completed_tasks: 0,
                threads: ArrayVec::new_array(),
                tasks: Slice::Stack(tasks),
            };
            scheduler.run();
        })
    }

    fn schedule(
        &mut self,
        task_id: usize,
        existing_thread_id: Option<usize>,
        main_channel: &SyncSender<TaskResult>,
    ) {
        let task = self.tasks[task_id].task;
        match existing_thread_id {
            Some(thread_id) => {
                let thread = &mut self.threads[thread_id];
                thread
                    .sender
                    .send(Message::Task(TaskMsg {
                        task,
                        thread_id,
                        task_id,
                    }))
                    .expect("Thread channel should not hangup!");
                thread.busy = true;
            }
            None => {
                let (sender, receiver) = mpsc::channel();
                let main_channel = main_channel.clone();
                // Vec has n-1 elements, so new thread would be nth element
                let new_thread_id = self.threads.len();
                self.scope.spawn(move || {
                    while let Ok(Message::Task(msg)) = receiver.recv() {
                        let result = catch_unwind(AssertUnwindSafe(msg.task));
                        main_channel
                            .send(TaskResult {
                                thread_id: msg.thread_id,
                                task_id: msg.task_id,
                                panic: result.err(),
                            })
                            .expect("Host channel should not hangup before child thread!");
                    }
                });
                sender
                    .send(Message::Task(TaskMsg {
                        task,
                        thread_id: new_thread_id,
                        task_id,
                    }))
                    .expect("Thread channel should not hangup!");
                self.threads.push(Thread { sender, busy: true });
            }
        }
    }

    fn run(mut self) {
        let (sender, receiver) = mpsc::sync_channel(self.tasks.len().min(50));
        for id in 0..self.tasks.len() {
            if self.tasks[id].num_parents == 0 {
                self.schedule(id, None, &sender);
            }
        }
        while self.completed_tasks < self.tasks.len()
            && let Ok(TaskResult {
                thread_id,
                task_id,
                panic,
            }) = receiver.recv()
        {
            if let Some(panic) = panic {
                resume_unwind(panic);
            }
            let task_children = &self.tasks[task_id].children;
            if task_children.is_empty() {
                self.threads[thread_id].busy = false;
                self.completed_tasks += 1;
                continue;
            }
            let mut child_executed = false;
            for &child_id in task_children.iter() {
                let child = &mut self.tasks[child_id];
                let all_complete = {
                    child.completed_parents += 1;
                    child.completed_parents == child.num_parents
                };
                if all_complete {
                    if !child_executed {
                        self.schedule(child_id, Some(thread_id), &sender);
                        // I'm sorry, little one.
                        child_executed = true;
                    } else {
                        let free_thread_id = self
                            .threads
                            .iter()
                            .enumerate()
                            .find_map(|(id, thread)| thread.busy.not().then_some(id));
                        self.schedule(child_id, free_thread_id, &sender);
                    }
                }
            }
            if !child_executed {
                self.threads[thread_id].busy = false;
            }
            self.completed_tasks += 1;
        }
    }
}

impl<'scope, 'env, const NUM_TASKS: usize> Drop for Scheduler<'scope, 'env, NUM_TASKS> {
    fn drop(&mut self) {
        for thread in self.threads.iter() {
            thread
                .sender
                .send(Message::Shutdown)
                .expect("Thread channel hung up before shutdown message!")
        }
    }
}

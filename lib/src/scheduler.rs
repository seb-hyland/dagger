use std::{
    any::Any,
    ops::Not,
    panic::{AssertUnwindSafe, RefUnwindSafe, UnwindSafe, catch_unwind, resume_unwind},
    sync::mpsc::{self, Sender, SyncSender},
    thread::Scope,
};

pub struct Scheduler<'scope, 'env, const NUM_TASKS: usize> {
    scope: &'scope Scope<'scope, 'env>,
    tasks: [Task<'scope>; NUM_TASKS],
    completed_tasks: usize,
    threads: Vec<Thread<'scope>>,
}

struct Thread<'scope> {
    sender: Sender<Message<'scope>>,
    busy: bool,
}

pub struct Task<'scope> {
    pub num_parents: u32,
    pub completed_parents: u32,
    pub children: &'scope [usize],
    pub task: &'scope (dyn Fn() + Send + Sync),
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

struct TaskMsg<'scope> {
    task: &'scope (dyn Fn() + Send + Sync),
    thread_id: usize,
    task_id: usize,
}

enum Message<'scope> {
    Task(TaskMsg<'scope>),
    Shutdown,
}

struct TaskResult {
    thread_id: usize,
    task_id: usize,
    panic: Option<Box<dyn Any + Send + 'static>>,
}

impl<'scope, 'env, const NUM_TASKS: usize> Scheduler<'scope, 'env, NUM_TASKS> {
    pub fn new(scope: &'scope Scope<'scope, 'env>, tasks: [Task<'scope>; NUM_TASKS]) -> Self {
        Scheduler {
            scope,
            tasks,
            completed_tasks: 0,
            threads: Vec::new(),
        }
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
                // Vec has n-1 elements, so new thread would be nth element
                let new_thread_id = self.threads.len();
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

    pub fn run(mut self) {
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
            task_children
                .iter()
                .enumerate()
                .for_each(|(child_num, &child_id)| {
                    let child = &mut self.tasks[child_id];
                    let all_complete = {
                        let completed_parents = &mut child.completed_parents;
                        *completed_parents += 1;
                        *completed_parents == child.num_parents
                    };
                    if all_complete {
                        if child_num == 0 {
                            self.schedule(child_id, Some(thread_id), &sender);
                        } else {
                            let free_thread_id = self
                                .threads
                                .iter()
                                .enumerate()
                                .find_map(|(id, thread)| thread.busy.not().then_some(id));
                            self.schedule(child_id, free_thread_id, &sender);
                        }
                    }
                });
            self.completed_tasks += 1;
        }
    }
}

impl<'scope, 'env, const NUM_TASKS: usize> Drop for Scheduler<'scope, 'env, NUM_TASKS> {
    fn drop(&mut self) {
        self.threads.iter().for_each(|thread| {
            thread
                .sender
                .send(Message::Shutdown)
                .expect("Thread channel hung up before shutdown message!")
        });
    }
}

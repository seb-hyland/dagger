use std::{
    any::Any,
    ops::Not,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    sync::mpsc::{self, Sender, SyncSender},
    thread::{self, Scope},
};

pub struct Scheduler<'scope, 'env> {
    scope: &'scope Scope<'scope, 'env>,
    tasks: Box<[Task<'scope>]>,
    completed_tasks: usize,
    threads: Vec<Thread<'scope>>,
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

impl<'scope, 'env> Scheduler<'scope, 'env> {
    pub fn execute<I>(tasks: I)
    where
        I: IntoIterator<Item = Task<'scope>>,
    {
        thread::scope(|s| {
            let tasks: Box<_> = tasks.into_iter().collect();
            let scheduler = Scheduler {
                scope: s,
                completed_tasks: 0,
                threads: Vec::with_capacity(tasks.len()),
                tasks,
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
        println!("Total threads: {}", self.threads.len());
    }
}

impl<'scope, 'env> Drop for Scheduler<'scope, 'env> {
    fn drop(&mut self) {
        self.threads.iter().for_each(|thread| {
            thread
                .sender
                .send(Message::Shutdown)
                .expect("Thread channel hung up before shutdown message!")
        });
    }
}

use dagger_lib::scheduler::{Scheduler, Task};

fn main() {
    let a_fn = || println!("a");
    let b_fn = || println!("b");
    let c_fn = || println!("c");
    let d_fn = || println!("d");

    let a_task = Task::new(0, &[1, 2], &a_fn);
    let b_task = Task::new(1, &[3], &b_fn);
    let c_task = Task::new(1, &[3], &c_fn);
    let d_task = Task::new(2, &[], &d_fn);

    Scheduler::execute([a_task, b_task, c_task, d_task]);
}

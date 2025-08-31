use std::{io, sync::Arc, thread};

use dagger::{
    CloneInner,
    process_data::ProcessData,
    result::{NodeError, NodeResult},
    scheduler::{Scheduler, Task},
};

fn main() {
    let a = ProcessData::default();
    let b = ProcessData::default();
    let c = ProcessData::default();
    let d = ProcessData::default();
    let e = ProcessData::default();
    let f = ProcessData::default();

    let a_fn = || a.set(Ok(Arc::new(sum(1, 5).unwrap())));
    let a_task = Task::new(0, &[1, 2], &a_fn);

    let b_fn = || {
        let a = unsafe { a.get() };
        b.set(Ok(Arc::new(mult(a.unwrap().clone_inner(), 5).unwrap())));
    };
    let b_task = Task::new(1, &[3], &b_fn);

    let c_fn = || {
        let a = unsafe { a.get() };
        c.set(Ok(Arc::new(mult(a.unwrap().clone_inner(), 10).unwrap())));
    };
    let c_task = Task::new(1, &[3, 4], &c_fn);

    let d_fn = || {
        let b = unsafe { b.get() };
        let c = unsafe { c.get() };
        d.set(Ok(Arc::new(
            div(b.unwrap().clone_inner(), c.unwrap().clone_inner()).unwrap(),
        )));
    };
    let d_task = Task::new(2, &[], &d_fn);

    let e_fn = || {
        let c = unsafe { c.get() };
        let c_str = c.unwrap().to_string();
        e.set(Ok(Arc::new(c_str)));
        panic!();
    };
    let e_task = Task::new(1, &[5], &e_fn);

    let f_fn = || {
        let e = unsafe { e.get() };
        println!("C as string is {}", e.unwrap().clone_inner());
        f.set(Ok(Arc::new(())));
    };
    let f_task = Task::new(1, &[], &f_fn);

    thread::scope(|s| {
        let tasks = [a_task, b_task, c_task, d_task, e_task, f_task];
        let scheduler = Scheduler::new(s, tasks);
        scheduler.run();
    });
    println!("{:?}", unsafe { d.get() });
}

fn sum(a: i32, b: i32) -> NodeResult<i32> {
    Ok(a + b)
}

fn mult(a: i32, b: i32) -> NodeResult<i32> {
    Ok(a * b)
}

fn div(a: i32, b: i32) -> NodeResult<f32> {
    if b == 0 {
        Err(NodeError::msg("Division by zero!"))
    } else {
        Ok(a as f32 / b as f32)
    }
}

fn double(_input: i32) -> NodeResult<i32> {
    Err(io::Error::last_os_error().into())
    // Ok(input * 2)
}

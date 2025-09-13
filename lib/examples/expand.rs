use dagger::prelude::*;
use std::io;
fn main() {
    let operation = {
        use dagger::__private::*;
        let execution_function = |write_path: Option<&::std::path::Path>, dot: &'static str| {
            let __private_sum_v = ProcessData::default();
            let __private_sum_v_fn = || {
                if true {
                    let () = ();
                    let process_value: NodeResult<_> = sum(3, 5);
                    let process_result = process_value.into_graph_result("sum_v");
                    __private_sum_v.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    __private_sum_v.set(Err(joined_err));
                }
            };
            let __private_sum_v_task = Task::new(0u32, &[1usize, 2usize], &__private_sum_v_fn);
            let __private_left_branch = ProcessData::default();
            let __private_left_branch_fn = || {
                let sum_v = unsafe { __private_sum_v.get() };
                if sum_v.is_ok() {
                    let (sum_v) = (sum_v.unwrap());
                    let process_value: NodeResult<_> = double(*sum_v);
                    let process_result = process_value.into_graph_result("left_branch");
                    __private_left_branch.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    if let Err(e) = sum_v {
                        e.push_error(&mut joined_err);
                    }
                    __private_left_branch.set(Err(joined_err));
                }
            };
            let __private_left_branch_task = Task::new(1u32, &[3usize], &__private_left_branch_fn);
            let __private_right_branch = ProcessData::default();
            let __private_right_branch_fn = || {
                let sum_v = unsafe { __private_sum_v.get() };
                if sum_v.is_ok() {
                    let (sum_v) = (sum_v.unwrap());
                    let process_value: NodeResult<_> = sum(*sum_v, 3);
                    let process_result = process_value.into_graph_result("right_branch");
                    __private_right_branch.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    if let Err(e) = sum_v {
                        e.push_error(&mut joined_err);
                    }
                    __private_right_branch.set(Err(joined_err));
                }
            };
            let __private_right_branch_task =
                Task::new(1u32, &[3usize, 4usize], &__private_right_branch_fn);
            let __private_mult_doubles = ProcessData::default();
            let __private_mult_doubles_fn = || {
                let left_branch = unsafe { __private_left_branch.get() };
                let right_branch = unsafe { __private_right_branch.get() };
                if left_branch.is_ok() && right_branch.is_ok() {
                    let (left_branch, right_branch) = (left_branch.unwrap(), right_branch.unwrap());
                    let process_value: NodeResult<_> = mult(*left_branch, *right_branch);
                    let process_result = process_value.into_graph_result("mult_doubles");
                    __private_mult_doubles.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    if let Err(e) = left_branch {
                        e.push_error(&mut joined_err);
                    }
                    if let Err(e) = right_branch {
                        e.push_error(&mut joined_err);
                    }
                    __private_mult_doubles.set(Err(joined_err));
                }
            };
            let __private_mult_doubles_task = Task::new(2u32, &[], &__private_mult_doubles_fn);
            let __private_div = ProcessData::default();
            let __private_div_fn = || {
                let right_branch = unsafe { __private_right_branch.get() };
                if right_branch.is_ok() {
                    let (right_branch) = (right_branch.unwrap());
                    let process_value: NodeResult<_> = div(*right_branch, 0);
                    let process_result = process_value.into_graph_result("div");
                    __private_div.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    if let Err(e) = right_branch {
                        e.push_error(&mut joined_err);
                    }
                    __private_div.set(Err(joined_err));
                }
            };
            let __private_div_task = Task::new(1u32, &[5usize], &__private_div_fn);
            let __private_reliant = ProcessData::default();
            let __private_reliant_fn = || {
                let div = unsafe { __private_div.get() };
                if div.is_ok() {
                    let (div) = (div.unwrap());
                    let process_value: NodeResult<_> = Ok(*div as i32);
                    let process_result = process_value.into_graph_result("reliant");
                    __private_reliant.set(process_result);
                } else {
                    let mut joined_err = GraphError::default();
                    if let Err(e) = div {
                        e.push_error(&mut joined_err);
                    }
                    __private_reliant.set(Err(joined_err));
                }
            };
            let __private_reliant_task = Task::new(1u32, &[], &__private_reliant_fn);
            Scheduler::execute([
                __private_sum_v_task,
                __private_left_branch_task,
                __private_right_branch_task,
                __private_mult_doubles_task,
                __private_div_task,
                __private_reliant_task,
            ]);
            let (mult_doubles, div) = (unsafe { __private_mult_doubles.get_owned() }, unsafe {
                __private_div.get_owned()
            });
            if let Some(path) = write_path {
                visualize_errors(
                    path,
                    &[
                        &mult_doubles.as_ref().map(|_| ()),
                        &div.as_ref().map(|_| ()),
                    ],
                    dot,
                );
            }
            (mult_doubles, div)
        };
        Graph::new(
            execution_function,
            "digraph {\ngraph [rankdir=\"TB\"]\nsum_v [shape=box label=\"sum_v\"]\nleft_branch [shape=box label=\"left_branch\"]\nsum_v -> left_branch\nright_branch [shape=box label=\"right_branch\"]\nsum_v -> right_branch\nmult_doubles [shape=box label=\"mult_doubles\"]\nleft_branch -> mult_doubles\nright_branch -> mult_doubles\ndiv [shape=box label=\"div\"]\nright_branch -> div\nreliant [shape=box label=\"reliant\"]\ndiv -> reliant\n\"___process_output\" [label=Output]\nmult_doubles -> \"___process_output\"\ndiv -> \"___process_output\"\n}\n",
        )
    };
    let (_a, _b) = operation.exe_visualize("hi.svg");
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
}

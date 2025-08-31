use std::io;

use dagger::prelude::*;

fn main() {
    let operation = dagger! {
        sum_v :: sum(3, 5);
        orphan :: Ok(5);
        left_branch :: double(sum_v.clone_inner());
        right_branch :: sum(sum_v.clone_inner(), orphan.clone_inner());
        mult_doubles :: mult(left_branch.clone_inner(), right_branch.clone_inner());
        div :: div(right_branch.clone_inner(), 0);
        reliant :: Ok(div.clone_inner() as i32);

        (mult_doubles, div)
    };
    // let _ = dbg!(operation.exe());

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
    // Ok(input * 2)
}

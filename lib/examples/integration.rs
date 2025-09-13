use std::{io, thread, time::Duration};

use dagger::prelude::*;

fn main() {
    let operation = dagger! {
        sum_v :: sum(3, 5);
        left_branch :: {
            thread::sleep(Duration::from_secs(3));
            double(*sum_v)
        };
        right_branch :: {
            println!("Finished right branch!");
            sum(*sum_v, 3)
        };
        mult_doubles :: {
            println!("Finished mult doubles!");
            mult(*left_branch, *right_branch)
        };
        div :: {
            println!("Finished div!");
            div(*right_branch, 5)
        };
        reliant :: {
            println!("Finished reliant!");
            Ok(*div as i32)
        };

        (mult_doubles, div, reliant)
    };
    // let _ = dbg!(operation.exe());

    let res = operation.exe_visualize("hi.svg");
    println!("{res:#?}");
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
    // Ok(_input * 2)
}

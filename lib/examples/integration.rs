use std::time::Instant;

use dagger::prelude::*;

fn main() {
    let start = Instant::now();
    let print_complete = |process_name: &'static str| {
        println!(
            "Finished {} at {}s",
            process_name,
            start.elapsed().as_secs_f32()
        )
    };
    let operation = dagger! {
        sum_v :: {
            // print_complete("sum_v");
            sum(3, 5)
        };
        left_branch :: {
            double(*sum_v)
        };
        right_branch :: {
            // print_complete("right_branch");
            div(*sum_v, 0)
        };
        cast :: {
            // print_complete("cast");
            Ok(*mult_doubles as f32)
        };
        div_v :: {
            // print_complete("div");
            div(*mult_doubles, 5)
        };
        double_v :: {
            // print_complete("double");
            double(*mult_doubles)
        };
        mult_doubles :: {
            // print_complete("mult_doubles");
            mult(*left_branch, *right_branch as i32)
        };
        double_left_branch :: {
            double(*left_branch)
        };
        return (cast, double_left_branch);
    };
    let res = operation.exe();
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
    // Err(std::io::Error::last_os_error().into())
    Ok(_input * 2)
}

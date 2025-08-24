use dagger::prelude::*;

fn main() {
    let operation = dagger! {
        sum :: sum(3, 5);
        double_1 :: double(sum.clone_inner());
        double_2 :: double(sum.clone_inner());
        mult_doubles :: mult(double_1.clone_inner(), double_2.clone_inner());
        div :: div(double_2.clone_inner(), 0);
        reliant :: Ok(div.clone_inner() as i32);
        (mult_doubles, div)
    };
    // let result = operation.exe();
    // let _ = dbg!(result);

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

fn double(input: i32) -> NodeResult<i32> {
    // Err(NodeError::msg("Failed"))
    Ok(input * 2)
}

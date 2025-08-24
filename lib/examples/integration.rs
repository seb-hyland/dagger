use std::ops::Deref;

use dagger::{
    dagger,
    result::{NodeError, NodeResult},
};

fn main() {
    let f32toi32 = |v: f32| -> NodeResult<i32> {
        Ok(v.floor() as i32)
        // Err("Some error")
    };

    let operation = dagger! {
        cic :: sum(3, 5);
        d :: mult(3, 5);
        e :: div(0, 0);
        f :: div(0, 0);
        g :: div(3, 5);
        h :: f32toi32(*g);
        i :: sum(3 + 14, *h);
        g_str_1 :: Ok(g.to_string());
        g_str :: Ok(g.to_string());
        g_str_array :: Ok([ g_str_1.deref().clone(), "Hi!".to_string(), cic.to_string() ]);
        out :: double(*h);
        (out, e, d, g_str_array)
    };
    let (a, b, c, d) = operation.exe();
    let err_dag = operation.visualize_errors([
        &a.map(|_| ()),
        &b.map(|_| ()),
        &c.map(|_| ()),
        &d.map(|_| ()),
    ]);
    println!("{}", err_dag);

    // let result = operation.exe();
    // let _ = dbg!(result);
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
    Ok(input * 2)
}

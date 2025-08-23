use std::ops::Deref;

use dagger::dagger;

fn main() {
    let f32toi32 = |v: f32| -> Result<i32, String> { Ok(v.floor() as i32) };

    let operation = dagger! {
        c :: sum(3, 5);
        d :: mult(3, 5);
        e :: sum(*c + 2, *d);
        f :: double(*d);
        g :: div(*e, 0);
        h :: f32toi32(*g);
        i :: sum(3 + 14, 5);
        g_str_1 :: g.to_string();
        g_str :: { g.to_string() };
        g_str_array :: [ g_str_1.deref().clone(), "Hi!".to_string() ];
        out :: double(*h);
        (out, e, d, g_str_array)
    };

    let result = operation.exe();
    let _ = dbg!(result);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn mult(a: i32, b: i32) -> i32 {
    a * b
}

fn div(a: i32, b: i32) -> Result<f32, &'static str> {
    if b == 0 {
        Err("Division by zero!")
    } else {
        Ok(a as f32 / b as f32)
    }
}

fn double(input: i32) -> i32 {
    input * 2
}

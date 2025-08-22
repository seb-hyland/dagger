use dagger::dagger;

fn main() {
    let input1 = || 3;
    let input2 = || 0;
    let f32toi32 = |v: f32| -> Result<i32, String> { Ok(v.floor() as i32) };
    let operation = dagger! {
        input1: input1(),
        input2: input2(),
        c: sum(input1, input2),
        d: mult(input1, input2),
        e: sum(c, d),
        f: double(d),
        g: div(e, f),
        h: f32toi32(g),
        i: sum(input1, input2),
        g_str: as_string(g),
        out: double(h),
        (out, e, d, g_str)
    };
    let result = operation.exe();
    println!("{}", operation.dot());
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

fn as_string(input: f32) -> String {
    input.to_string()
}

#![feature(custom_inner_attributes)]
#![rustfmt::skip]

use dagger::dagger;

fn main() {
    let input1 = || 3;
    let input2 = || 5;
    let f32toi32 = |v: f32| v.floor() as i32;
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
        print_g: print_string(g_str),
        out: double(h),
        (out, e, c, g_str)
    };
    let result = operation.exe();
    dbg!(result);
}

fn sum(a: i32, b: i32) -> i32 { a + b }

fn mult(a: i32, b: i32) -> i32 { a * b }

fn div(a: i32, b: i32) -> f32 { a as f32 / b as f32 }

fn double(input: i32) -> i32 { input * 2 }

fn as_string(input: f32) -> String { input.to_string() }

fn print_string(input: String) { println!("{input}") }

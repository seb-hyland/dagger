#![feature(custom_inner_attributes)]
#![rustfmt::skip]

use dagger::graph;

fn main() {
    let v = graph! {
        a: input1(),
        b: input2(),
        c: sum(a, b),
        d: mult(a, b),
        e: sum(c, d),
        f: double(d),
        g: div(e, f),
        g
    }.exe();
    println!("{v}");
}

fn input1() -> i32 { 3 }

fn input2() -> i32 { 5 }

fn sum(a: i32, b: i32) -> i32 { a + b }

fn mult(a: i32, b: i32) -> i32 { a * b }

fn div(a: i32, b: i32) -> f32 { a as f32 / b as f32 }

fn double(input: i32) -> i32 { input * 2 }

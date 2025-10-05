use dagger_lib::prelude::*;
use std::io::{Write, stdin, stdout};

fn main() {
    let graph = dagger! {
        a :: Ok(5);
        b :: Ok(*a);
        return b;
    };
    let dagger_output = graph.exe_visualize("visualization.svg");
    // let dagger_output = graph.dot();
    // let dagger_output = graph.exe();
    println!("dagger_output = {dagger_output:?}");
}

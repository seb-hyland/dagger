use dagger::prelude::*;

fn main() {
    let graph = dagger! {
        a :: Ok(5);
        b :: Ok(a + 3);
        (a, b)
    };
    let dagger_output = graph.exe();
    println!("dagger_output = {dagger_output:?}");
}

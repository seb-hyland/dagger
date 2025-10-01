use dagger_lib::prelude::*;

fn main() {
    let graph = dagger! {
        f :: Ok(d + 5);
        a :: Ok(5);
        e :: Ok(d + 5);
        b :: Err(NodeError::msg("failed process"));
        d :: Ok(b + c);
        c :: Ok(a + 5);

        return (c, f);
    };
    let dagger_output = graph.exe_visualize("visualization.svg");
    // let dagger_output = graph.exe();
    println!("dagger_output = {dagger_output:?}");
}

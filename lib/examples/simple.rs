use dagger_lib::prelude::*;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    hint::black_box,
};

fn expensive(input: i32) -> NodeResult<i32> {
    black_box({
        let mut hasher = DefaultHasher::new();
        for i in 0..1_000 {
            i.hash(&mut hasher);
        }
        hasher.finish()
    });
    Ok(input)
}
fn main() {
    fn tleap(_: (), _: ()) -> NodeResult<()> {
        Ok(())
    }
    fn analysis_process_1(_: &()) -> NodeResult<()> {
        Ok(())
    }
    fn analysis_process_2(_: &()) -> NodeResult<()> {
        Ok(())
    }
    fn analysis_process_3(_: &()) -> NodeResult<()> {
        Ok(())
    }
    let (path, molecule_name) = ((), ());
    println!(
        "{}",
        dagger! {
            tleap_out :: tleap(path, molecule_name);
            analysis_1 :: analysis_process_1(tleap_out);
            analysis_2 :: analysis_process_2(tleap_out);
            analysis_3 :: analysis_process_3(analysis_2);
        }
        .dot()
    );
    // let dagger_output = graph.exe();
    // println!("{dagger_output:?}");
}

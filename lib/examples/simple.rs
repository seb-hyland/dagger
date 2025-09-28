use dagger::prelude::*;
use std::{
    hash::{DefaultHasher, Hash, Hasher},
    hint::black_box,
};

fn expensive() {
    black_box({
        let mut hasher = DefaultHasher::new();
        for i in 0..1_000 {
            i.hash(&mut hasher);
        }
        hasher.finish()
    });
}
fn main() {
    let graph = dagger! {
        a :: {
            expensive();
            Ok(5)
        };
        b :: {
            expensive();
            Ok(a + 3)
        };
        c :: {
            expensive();
            Ok(a + 5)
        };
        d :: {
            expensive();
            Ok(b + c)
        };
        e :: {
            expensive();
            Ok(d + 3)
        };
        f :: {
            expensive();
            Ok(d + 5)
        };
    };
    let dagger_output = graph.exe();
    println!("dagger_output = {dagger_output:?}");
}

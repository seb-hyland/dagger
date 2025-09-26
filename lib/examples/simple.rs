use dagger::prelude::*;
use std::hint::black_box;

fn expensive() -> i32 {
    black_box({
        let mut v = 0;
        for i in 0..10000 {
            v += i;
        }
        v
    })
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

use dagger_lib::prelude::*;

fn main() {
    let vec = vec![1, 3, 4, 2, 3];
    parallelize(vec, |i| println!("{i}"));
}

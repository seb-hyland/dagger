#![feature(unsafe_cell_access)]
#![feature(ptr_as_ref_unchecked)]
#![allow(dead_code)]

pub use dagger_macros::graph;
pub mod data;

pub struct Graph<T> {
    func: fn() -> T,
}

impl<T> Graph<T> {
    pub fn new(func: fn() -> T) -> Graph<T> {
        Graph { func }
    }

    pub fn exe(&self) -> T {
        (self.func)()
    }
}

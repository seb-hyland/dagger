#![feature(unsafe_cell_access)]
#![feature(ptr_as_ref_unchecked)]
#![allow(dead_code)]

pub mod data;
pub mod result;
pub use dagger_macros::operation;
use std::{fs::write, io, path::Path};

pub struct Graph<T, F: Fn() -> T> {
    func: F,
    dot: &'static str,
    svg: &'static str,
}

impl<T, F: Fn() -> T> Graph<T, F> {
    pub fn new(func: F, dot: &'static str, svg: &'static str) -> Graph<T, F> {
        Graph { func, dot, svg }
    }

    pub fn exe(&self) -> T {
        (self.func)()
    }

    pub fn dot(&self) -> &'static str {
        self.dot
    }

    pub fn svg(&self) -> &'static str {
        self.svg
    }

    pub fn savefig<P: AsRef<Path>>(&self, path: P) -> Result<(), io::Error> {
        write(path, self.svg)
    }
}

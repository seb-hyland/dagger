#![feature(unsafe_cell_access)]
#![feature(ptr_as_ref_unchecked)]
#![allow(dead_code)]

pub mod prelude;
pub mod process_data;
#[cfg(feature = "visualize")]
mod visualization;
pub use dagger_macros::dagger;
use std::{
    error::Error,
    ops::{Deref, DerefMut},
    sync::Arc,
};

pub struct Graph<T, F: Fn() -> T> {
    func: F,
    #[cfg(feature = "visualize")]
    dot: &'static str,
}

impl<T, F: Fn() -> T> Graph<T, F> {
    #[cfg(not(feature = "visualize"))]
    pub fn new(func: F) -> Graph<T, F> {
        Graph { func }
    }

    #[cfg(feature = "visualize")]
    pub fn new(func: F, dot: &'static str) -> Graph<T, F> {
        Graph { func, dot }
    }

    pub fn exe(&self) -> T {
        (self.func)()
    }

    #[cfg(feature = "visualize")]
    pub fn dot(&self) -> &'static str {
        self.dot
    }
}

pub type ProcessResult<T> = Result<T, ProcessError>;

#[derive(Debug, Default)]
pub struct ProcessError(ProcessErrorInner);
type ProcessErrorInner = Vec<(&'static str, Arc<dyn Error + Send + Sync>)>;

impl Deref for ProcessError {
    type Target = ProcessErrorInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ProcessError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait IntoProcessResult<T> {
    fn into_process_result(self, node: &'static str) -> ProcessResult<T>;
}
impl<T> IntoProcessResult<T> for T {
    fn into_process_result(self, _node: &'static str) -> ProcessResult<T> {
        Ok(self)
    }
}
impl<T, E: Error + 'static + Send + Sync> IntoProcessResult<T> for Result<T, E> {
    fn into_process_result(self, node: &'static str) -> ProcessResult<T> {
        match self {
            Err(e) => Err(ProcessError(vec![(node, Arc::new(e))])),
            Ok(v) => Ok(v),
        }
    }
}

pub trait PushProcessError<E> {
    fn push_error(&self, node: &'static str, error: &mut ProcessError);
}
impl<E: Error + 'static + Send + Sync + Clone> PushProcessError<E> for E {
    fn push_error(&self, node: &'static str, error: &mut ProcessError) {
        error.push((node, Arc::new(self.clone())));
    }
}
impl PushProcessError<ProcessError> for ProcessError {
    fn push_error(&self, _node: &'static str, error: &mut ProcessError) {
        error.extend(self.iter().cloned());
    }
}

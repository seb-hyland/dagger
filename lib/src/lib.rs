#![allow(clippy::macro_metavars_in_unsafe)]
use std::path::Path;

#[doc(hidden)]
pub mod __private;
/// Additional runtime parallelization APIs
pub mod parallelize;
/// Public exports
///
/// dagger should be imported into projects by importing the entire prelude:
/// ```rust
/// use dagger::prelude::*
/// ```
pub mod prelude;
/// Data structures to store node output values
pub mod process_data;
/// Error tracing, reporting, and handling
pub mod result;
#[doc(hidden)]
pub mod scheduler;
#[cfg(feature = "visualize")]
mod visualization;

pub use dagger_macros::dagger;

/// An execution DAG constructed by [`dagger`]
pub struct Graph<T, F: Fn(Option<&Path>, &'static str) -> T> {
    func: F,
    #[cfg(feature = "visualize")]
    dot: &'static str,
}

#[cfg(not(feature = "visualize"))]
impl<T, F: Fn(Option<&Path>, &'static str) -> T> Graph<T, F> {
    pub fn new(func: F) -> Graph<T, F> {
        Graph { func }
    }

    pub fn exe(&self) -> T {
        (self.func)(None, "")
    }
}

#[cfg(feature = "visualize")]
impl<T, F: Fn(Option<&Path>, &'static str) -> T> Graph<T, F> {
    #[doc(hidden)]
    pub fn new(func: F, dot: &'static str) -> Graph<T, F> {
        Graph { func, dot }
    }

    /// Execute the [`Graph`]
    pub fn exe(&self) -> T {
        (self.func)(None, "")
    }

    /// Execute the [`Graph`], saving a visualization at `path`
    pub fn exe_visualize<P: AsRef<Path>>(&self, path: P) -> T {
        (self.func)(Some(path.as_ref()), self.dot)
    }

    /// Obtain the GraphViz DOT representation of the [`Graph`]
    pub fn dot(&self) -> &'static str {
        self.dot
    }
}

/// Trust me, I'm right 😎
///
/// An alias for the `unsafe` keyword, designed for programmers with an inflated ego
/// ## Example:
/// ```rust
/// trust_me_bro! {
///     // Some unsafe operations 🤢
/// }
/// ```
#[macro_export]
macro_rules! trust_me_bro {
    ($($token:tt)*) => {
        unsafe { $($token)* }
    };
}

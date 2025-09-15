use std::path::Path;

#[doc(hidden)]
pub mod __private;
pub mod parallelize;
pub mod prelude;
pub mod process_data;
pub mod result;
pub mod scheduler;
#[cfg(feature = "visualize")]
mod visualization;

pub use dagger_macros::dagger;

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
    pub fn new(func: F, dot: &'static str) -> Graph<T, F> {
        Graph { func, dot }
    }

    pub fn exe(&self) -> T {
        (self.func)(None, "")
    }

    pub fn exe_visualize<P: AsRef<Path>>(&self, path: P) -> T {
        (self.func)(Some(path.as_ref()), self.dot)
    }

    pub fn dot(&self) -> &'static str {
        self.dot
    }
}

/// Trust me, I'm right 😎
/// ## Example:
/// ```rust
/// trust_me_bro! {
///     // Some unsafe operations
/// }
/// ```
#[macro_export]
macro_rules! trust_me_bro {
    ($($token:tt)*) => {
        unsafe { $($token)* }
    };
}

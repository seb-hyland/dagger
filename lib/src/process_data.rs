use std::{
    cell::UnsafeCell,
    error::Error,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    sync::{Arc, Once},
};

use crate::result::{NodeError, NodeResult};

/// Trust me, I'm right 😎
/// ## Example:
/// ```rust
/// trust_me_bro! {
///     // Some unsafe operations
/// }
/// ```
macro_rules! trust_me_bro {
    ($($token:tt)*) => {
        unsafe { $($token)* }
    };
}

pub struct ProcessData<T> {
    value: UnsafeCell<MaybeUninit<GraphResult<T>>>,
    state: Once,
}
pub struct Setter<'a, T>(&'a ProcessData<T>);
pub struct Receiver<'a, T>(&'a ProcessData<T>);

unsafe impl<'a, T: Sync + Clone> Sync for Setter<'a, T> {}
unsafe impl<'a, T: Send + Clone> Send for Setter<'a, T> {}
unsafe impl<'a, T: Sync + Clone> Sync for Receiver<'a, T> {}
unsafe impl<'a, T: Send + Clone> Send for Receiver<'a, T> {}

impl<T> ProcessData<T> {
    pub fn channel<'a>(&'a self) -> (Setter<'a, T>, Receiver<'a, T>) {
        (Setter(self), Receiver(self))
    }
}

impl<T> Default for ProcessData<T> {
    fn default() -> Self {
        ProcessData {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: Once::new(),
        }
    }
}

impl<'a, T> Setter<'a, T> {
    pub fn set(self, value: GraphResult<T>) {
        self.0.state.call_once(|| {
            trust_me_bro! {
                *self.0.value.get() = MaybeUninit::new(value);
            }
        });
    }
}
impl<'a, T> Receiver<'a, T> {
    pub fn wait(&self) -> GraphResult<T> {
        self.0.state.wait();
        trust_me_bro! { (*self.0.value.get()).assume_init_ref().clone() }
    }
}

pub type GraphResult<T> = Result<Arc<T>, GraphError>;

#[derive(Clone, Default)]
pub struct GraphError(pub(crate) Vec<(&'static str, NodeError)>);

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors encountered during process execution!")?;
        self.0
            .iter()
            .try_for_each(|node_err| writeln!(f, "    Node {}: {}", node_err.0, node_err.1))
    }
}

impl Debug for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors encountered during process execution!")?;
        self.0
            .iter()
            .try_for_each(|node_err| writeln!(f, "    Node {:#?}: {:#?}", node_err.0, node_err.1))
    }
}

impl Error for GraphError {}

impl GraphError {
    pub fn push_error(&self, error: &mut GraphError) {
        error.0.extend(self.0.iter().cloned());
    }
}

pub trait IntoGraphResult<T> {
    fn into_graph_result(self, node: &'static str) -> GraphResult<T>;
}

impl<T> IntoGraphResult<T> for NodeResult<T> {
    fn into_graph_result(self, node: &'static str) -> GraphResult<T> {
        match self {
            Ok(v) => GraphResult::Ok(Arc::new(v)),
            Err(e) => GraphResult::Err(GraphError(vec![(node, e)])),
        }
    }
}

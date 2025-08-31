use std::{
    cell::UnsafeCell,
    error::Error,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    ptr,
    sync::Arc,
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

pub struct ProcessData<T>(UnsafeCell<MaybeUninit<GraphResult<T>>>);

unsafe impl<T: Sync + Clone> Send for ProcessData<T> {}
unsafe impl<T: Sync + Clone> Sync for ProcessData<T> {}

impl<T> Default for ProcessData<T> {
    fn default() -> Self {
        ProcessData(UnsafeCell::new(MaybeUninit::uninit()))
    }
}

impl<T> ProcessData<T> {
    pub fn set(&self, value: GraphResult<T>) {
        trust_me_bro! {
            ptr::write(self.0.get(), MaybeUninit::new(value));
        }
    }

    /// ...
    /// # Safety
    /// Address must be initialized
    pub unsafe fn get(&self) -> GraphResult<T> {
        trust_me_bro! { (*self.0.get()).assume_init_ref().clone() }
    }
}

pub type GraphResult<T> = Result<Arc<T>, GraphError>;

#[derive(Clone, Default)]
pub struct GraphError(pub(crate) Vec<(&'static str, NodeError)>);

impl Display for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|node_err| {
            writeln!(f, "Node {} failed with error {}", node_err.0, node_err.1)
        })
    }
}

impl Debug for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct NodeErrorTuple<'a>(&'static str, &'a NodeError);
        impl<'a> Debug for NodeErrorTuple<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = format!("Node[{}]", self.0);
                let mut dbg = f.debug_struct(&name);
                dbg.field("Location", &self.1);
                dbg.finish()
            }
        }

        let mut list = f.debug_list();
        self.0.iter().for_each(|node_err| {
            let err_struct = NodeErrorTuple(node_err.0, &node_err.1);
            list.entry(&err_struct);
        });
        list.finish()
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

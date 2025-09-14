use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
    panic::Location,
    sync::Arc,
};

pub type NodeResult<T> = Result<T, NodeError>;
pub trait IntoGraphResult<T> {
    fn into_graph_result(self, node: &'static str) -> GraphResult<T>;
}
impl<T> IntoGraphResult<T> for NodeResult<T> {
    fn into_graph_result(self, node: &'static str) -> GraphResult<T> {
        match self {
            Ok(v) => GraphResult::Ok(v),
            Err(e) => GraphResult::Err(GraphError::new(vec![(node, e)])),
        }
    }
}

#[derive(Clone)]
pub struct NodeError {
    error: Arc<dyn Error + Send + Sync + 'static>,
    caller: &'static Location<'static>,
}

impl Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" at {}", self.error, self.caller)
    }
}

impl Debug for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = format!("{}", self.caller);
        let mut dbg = f.debug_struct(&location);
        dbg.field("Error", &*self.error);
        dbg.finish()
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for NodeError {
    #[track_caller]
    fn from(value: E) -> Self {
        let error = Arc::new(value);
        let caller = Location::caller();
        NodeError { error, caller }
    }
}

impl AsRef<dyn Error> for NodeError {
    fn as_ref(&self) -> &(dyn Error + 'static) {
        &self.error
    }
}

struct MsgError<M: Display + Debug + Send + Sync>(M);
impl<M: Display + Debug + Send + Sync> Display for MsgError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<M: Display + Debug + Send + Sync> Debug for MsgError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}
impl<M: Display + Debug + Send + Sync> Error for MsgError<M> {}

impl NodeError {
    #[track_caller]
    pub fn msg<M: Display + Debug + Send + Sync + 'static>(msg: M) -> NodeError {
        let error = Arc::new(MsgError(msg));
        let caller = Location::caller();
        NodeError { error, caller }
    }
}

pub type GraphResult<T> = Result<T, GraphError>;

#[derive(Clone, Default)]
pub struct GraphError(Vec<(&'static str, NodeError)>);
impl GraphError {
    pub(crate) fn new(err: Vec<(&'static str, NodeError)>) -> Self {
        GraphError(err)
    }
}
impl Deref for GraphError {
    type Target = Vec<(&'static str, NodeError)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

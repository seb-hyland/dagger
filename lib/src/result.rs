use std::{
    error::Error,
    fmt::{Debug, Display},
    panic::Location,
    sync::Arc,
};

pub type NodeResult<T> = Result<T, NodeError>;

#[derive(Clone)]
pub struct NodeError {
    error: Arc<dyn Error>,
    caller: &'static Location<'static>,
}

impl Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: {}", self.caller, self.error)
    }
}

impl Debug for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: {:#?}", self.caller, self.error)
    }
}

impl<E: Error + 'static> From<E> for NodeError {
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

#[derive(Debug)]
struct MsgError(String);
impl Display for MsgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for MsgError {}

impl NodeError {
    #[track_caller]
    pub fn msg<M: ToString>(msg: M) -> NodeError {
        let error = Arc::new(MsgError(msg.to_string()));
        let caller = Location::caller();
        NodeError { error, caller }
    }
}

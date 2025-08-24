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

struct MsgError<M: Display + Debug>(M);
impl<M: Display + Debug> Display for MsgError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<M: Display + Debug> Debug for MsgError<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}
impl<M: Display + Debug> Error for MsgError<M> {}

impl NodeError {
    #[track_caller]
    pub fn msg<M: Display + Debug + 'static>(msg: M) -> NodeError {
        let error = Arc::new(MsgError(msg));
        let caller = Location::caller();
        NodeError { error, caller }
    }
}

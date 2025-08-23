use std::{
    cell::UnsafeCell,
    error::Error,
    fmt::{Debug, Display},
    mem::MaybeUninit,
    sync::{Arc, Once},
};

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
    value: UnsafeCell<MaybeUninit<ProcessResult<T>>>,
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
    pub fn set(self, value: ProcessResult<T>) {
        self.0.state.call_once(|| {
            trust_me_bro! {
                *self.0.value.get() = MaybeUninit::new(value);
            }
        });
    }
}
impl<'a, T> Receiver<'a, T> {
    pub fn wait(&self) -> ProcessResult<T> {
        self.0.state.wait();
        trust_me_bro! { (*self.0.value.get()).assume_init_ref().clone() }
    }
}

pub type ProcessResult<T> = Result<Arc<T>, ProcessError>;

#[derive(Default, Clone)]
pub struct ProcessError(pub(crate) Vec<Arc<NodeError>>);
pub(crate) struct NodeError {
    pub(crate) node: &'static str,
    display: String,
    debug: String,
}
impl NodeError {
    fn new<E: Display + Debug>(node: &'static str, error: &E) -> Self {
        NodeError {
            node,
            display: error.to_string(),
            debug: format!("{error:#?}"),
        }
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors encountered during process execution!")?;
        self.0.iter().try_for_each(|node_err| {
            writeln!(f, "    Node {}: {}", node_err.node, node_err.display)
        })
    }
}

impl Debug for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors encountered during process execution!")?;
        self.0
            .iter()
            .try_for_each(|node_err| writeln!(f, "    Node {}: {}", node_err.node, node_err.debug))
    }
}

impl Error for ProcessError {}

impl ProcessError {
    pub fn push_error(&self, _node: &'static str, error: &mut ProcessError) {
        error.0.extend(self.0.iter().cloned());
    }
}

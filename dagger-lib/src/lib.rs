pub use dagger_macros::graph;
use std::sync::{Arc, Condvar, Mutex};

pub struct Graph<T> {
    func: fn() -> T,
}

impl<T> Graph<T> {
    pub fn new(func: fn() -> T) -> Graph<T> {
        Graph { func }
    }

    pub fn exe(&self) -> T {
        (self.func)()
    }
}

pub struct ProcessData<T: Clone> {
    inner: Arc<ProcessDataInner<T>>,
}

struct ProcessDataInner<T: Clone> {
    data: Mutex<Option<T>>,
    condvar: Condvar,
}

impl<T: Clone> Clone for ProcessData<T> {
    fn clone(&self) -> Self {
        ProcessData {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Clone> ProcessData<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ProcessData<T> {
        ProcessData {
            inner: Arc::new(ProcessDataInner {
                data: Mutex::new(None),
                condvar: Condvar::new(),
            }),
        }
    }

    pub fn set(&self, value: T) {
        *self.inner.data.lock().expect("Mutexes should not poison!") = Some(value);
        self.inner.condvar.notify_all();
    }

    pub fn wait(&self) -> T {
        let guard = self.inner.data.lock().expect("Mutexes should not poison!");
        let guard = self
            .inner
            .condvar
            .wait_while(guard, |val| val.is_none())
            .expect("Mutexes should not poison!");
        guard.as_ref().unwrap().clone()
    }
}

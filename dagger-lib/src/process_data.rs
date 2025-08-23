use std::{cell::UnsafeCell, mem::MaybeUninit, ops::Deref, sync::Once};

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

pub struct ProcessData<T: Clone> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: Once,
}
pub struct Setter<'a, T: Clone>(&'a ProcessData<T>);
pub struct Receiver<'a, T: Clone>(&'a ProcessData<T>);

unsafe impl<'a, T: Sync + Clone> Sync for Setter<'a, T> {}
unsafe impl<'a, T: Send + Clone> Send for Setter<'a, T> {}
unsafe impl<'a, T: Sync + Clone> Sync for Receiver<'a, T> {}
unsafe impl<'a, T: Send + Clone> Send for Receiver<'a, T> {}

impl<T: Clone> ProcessData<T> {
    pub fn channel<'a>(&'a self) -> (Setter<'a, T>, Receiver<'a, T>) {
        (Setter(self), Receiver(self))
    }
}

impl<T: Clone> Default for ProcessData<T> {
    fn default() -> Self {
        ProcessData {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: Once::new(),
        }
    }
}
impl<'a, T: Clone> Deref for Setter<'a, T> {
    type Target = &'a ProcessData<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a, T: Clone> Deref for Receiver<'a, T> {
    type Target = &'a ProcessData<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Clone> Setter<'a, T> {
    pub fn set(self, value: T) {
        self.state.call_once(|| {
            trust_me_bro! {
                *self.value.get() = MaybeUninit::new(value);
            }
        });
    }
}
impl<'a, T: Clone> Receiver<'a, T> {
    pub fn wait(&self) -> T {
        self.state.wait();
        trust_me_bro! { (*self.value.get()).assume_init_ref().clone() }
    }
}

use std::{cell::UnsafeCell, mem::MaybeUninit, ptr};

use crate::result::{GraphError, GraphResult};

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
    pub unsafe fn get(&self) -> Result<&T, &GraphError> {
        trust_me_bro! { (*self.0.get()).assume_init_ref().as_ref() }
    }

    /// ...
    /// # Safety
    /// Address must be initialized
    pub unsafe fn get_owned(self) -> Result<T, GraphError> {
        trust_me_bro! { self.0.into_inner().assume_init() }
    }
}

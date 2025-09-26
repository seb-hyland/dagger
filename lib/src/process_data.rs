use crate::{
    result::{GraphError, GraphResult},
    trust_me_bro,
};
use std::{cell::UnsafeCell, mem::MaybeUninit, ptr};

pub struct ProcessData<T>(UnsafeCell<MaybeUninit<GraphResult<T>>>);

unsafe impl<T: Sync + Clone> Send for ProcessData<T> {}
unsafe impl<T: Sync + Clone> Sync for ProcessData<T> {}

impl<T> Default for ProcessData<T> {
    fn default() -> Self {
        const { ProcessData(UnsafeCell::new(MaybeUninit::uninit())) }
    }
}

impl<T> ProcessData<T> {
    pub fn set(&self, value: GraphResult<T>) {
        trust_me_bro! { ptr::write(self.0.get(), MaybeUninit::new(value)) }
    }

    /// ...
    /// # Safety
    /// Address must be initialized
    pub const unsafe fn get(&self) -> Result<&T, &GraphError> {
        trust_me_bro! { (*self.0.get()).assume_init_ref().as_ref() }
    }

    /// ...
    /// # Safety
    /// Address must be initialized
    pub unsafe fn get_owned(self) -> Result<T, GraphError> {
        let manual_drop = std::mem::ManuallyDrop::new(self);
        trust_me_bro! { std::ptr::read(&manual_drop.0).into_inner().assume_init() }
    }
}

impl<T> Drop for ProcessData<T> {
    fn drop(&mut self) {
        unsafe { self.0.get_mut().assume_init_drop() }
    }
}

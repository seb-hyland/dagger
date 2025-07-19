use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicU32,
        Ordering::{Acquire, Relaxed},
    },
};

pub struct ProcessData<T: Clone> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: Patience,
}
unsafe impl<T: Clone + Sync> Sync for ProcessData<T> {}
unsafe impl<T: Clone + Send> Send for ProcessData<T> {}

impl<T: Clone> ProcessData<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ProcessData<T> {
        ProcessData {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: Patience::new(),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.as_mut_unchecked() = MaybeUninit::new(value);
        }
        self.state.ready();
    }

    pub fn wait(&self) -> T {
        self.state.wait_loaded();
        unsafe { self.value.get().as_ref_unchecked().assume_init_read() }
    }
}

struct Patience {
    inner: AtomicU32,
}

impl Patience {
    fn new() -> Patience {
        Patience {
            inner: AtomicU32::new(0),
        }
    }

    fn ready(&self) {
        self.inner.store(1, Relaxed);
        waiter::wake_all(&self.inner as *const AtomicU32);
    }

    fn wait_loaded(&self) {
        if self.inner.load(Acquire) != 0 {
            return;
        }
        waiter::wait_on(&self.inner);
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod waiter {
    use core::{ptr, sync::atomic::AtomicU32};
    use libc::{FUTEX_PRIVATE_FLAG, FUTEX_WAIT, SYS_futex, syscall, timespec};

    #[inline]
    pub(crate) fn wait_on(cond: &AtomicU32) {
        unsafe {
            syscall(
                SYS_futex,
                cond,
                FUTEX_WAIT | FUTEX_PRIVATE_FLAG,
                0,
                ptr::null::<timespec>(),
            );
        };
    }

    #[inline]
    pub(crate) fn wake_all(ptr: *const AtomicU32) {
        unsafe {
            libc::syscall(
                libc::SYS_futex,
                ptr,
                libc::FUTEX_WAKE | libc::FUTEX_PRIVATE_FLAG,
                i32::MAX,
            );
        };
    }
}

#[cfg(target_os = "freebsd")]
mod waiter {
    use core::{ptr::null_mut, sync::atomic::AtomicU32};
    use libc::{_umtx_op, UMTX_OP_WAIT_UINT_PRIVATE, UMTX_OP_WAKE_PRIVATE, c_ulong, c_void};

    #[inline]
    pub(crate) fn wait_on(cond: &AtomicU32) {
        unsafe {
            libc::_umtx_op(
                cond as *const AtomicU32 as *mut c_void,
                UMTX_OP_WAIT_UINT_PRIVATE,
                0 as libc::c_ulong,
                null_mut(),
                null_mut(),
            );
        };
    }

    #[inline]
    pub(crate) fn wake_all(ptr: *const AtomicU32) {
        unsafe {
            _umtx_op(
                ptr as *mut c_void,
                UMTX_OP_WAKE_PRIVATE,
                i32::MAX as c_ulong,
                null_mut(),
                null_mut(),
            );
        };
    }
}

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "watchos"))]
mod waiter {
    use core::{
        ffi::c_void,
        sync::atomic::{AtomicU32, Ordering::Relaxed},
    };

    #[link(name = "c++")]
    extern "C" {
        #[link_name = "_ZNSt3__123__libcpp_atomic_monitorEPVKv"]
        fn __libcpp_atomic_monitor(ptr: *const c_void) -> i64;

        #[link_name = "_ZNSt3__120__libcpp_atomic_waitEPVKvx"]
        fn __libcpp_atomic_wait(ptr: *const c_void, monitor: i64);

        #[link_name = "_ZNSt3__123__cxx_atomic_notify_allEPVKv"]
        fn __cxx_atomic_notify_all(ptr: *const c_void);
    }

    #[inline]
    pub(crate) fn wait_on(cond: &AtomicU32) {
        let monitor = unsafe { __libcpp_atomic_monitor(cond.cast()) };
        if cond.load(Relaxed) != 0 {
            return;
        }
        unsafe { __libcpp_atomic_wait(cond.cast(), monitor) };
    }

    #[inline]
    pub(crate) fn wake_all(ptr: *const AtomicU32) {
        unsafe { __cxx_atomic_notify_all(ptr.cast()) };
    }
}

#[cfg(target_os = "windows")]
mod waiter {
    use core::sync::atomic::AtomicU32;
    use windows_sys::Win32::System::Threading::{INFINITE, WaitOnAddress, WakeByAddressAll};

    #[inline]
    pub(crate) fn wait_on(cond: &AtomicU32) {
        let ptr: *const AtomicU32 = cond;
        let expected_ptr: *const u32 = 0;
        unsafe { WaitOnAddress(ptr.cast(), expected_ptr.cast(), 4, INFINITE) };
    }

    #[inline]
    pub(crate) fn wake_all(ptr: *const AtomicU32) {
        unsafe { WakeByAddressAll(ptr.cast()) };
    }
}

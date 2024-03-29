use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::Ordering;
use core::task::Waker;
use portable_atomic::AtomicPtr;

/// Utility struct to register and wake a waker.
pub struct AtomicWaker {
    waker: AtomicPtr<()>,
}

impl AtomicWaker {
    /// Create a new `AtomicWaker`.
    pub const fn new() -> Self {
        Self {
            waker: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Register a waker. Overwrites the previous waker, if any.
    pub fn register(&self, w: &Waker) {
        self.waker.store(w.as_turbo_ptr().as_ptr() as _, Ordering::Release);
    }

    /// Wake the registered waker, if any.
    pub fn wake(&self) {
        if let Some(ptr) = NonNull::new(self.waker.load(Ordering::Acquire)) {
            unsafe { Waker::from_turbo_ptr(ptr) }.wake();
        }
    }
}

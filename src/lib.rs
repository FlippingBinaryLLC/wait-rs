#![no_std]
#![doc = include_str!("../README.md")]

use core::{
    future::Future,
    task::{Context, Poll, Waker},
};

#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use alloc::boxed::Box;
#[cfg(feature = "std")]
use std::{sync::Arc, task::Wake, thread};

#[cfg(not(feature = "std"))]
use core::{
    pin::Pin,
    task::{RawWaker, RawWakerVTable},
};

#[cfg(not(feature = "std"))]
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| RawWaker::new(core::ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

#[cfg(feature = "std")]
struct ThreadWaker {
    thread: std::thread::Thread,
}

#[cfg(feature = "std")]
impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.thread.unpark();
    }
}

/// The `Waitable` trait declares the `.wait()` method.
///
/// This trait is implemented for all types that implement the [`Future`]
/// trait. All `async` functions return a `Future`, so this attaches the
/// `.wait()` method to every `async` function. When called, the `.wait()`
/// puts the thread to sleep until the `Future` is ready to return a value.
pub trait Waitable: sealed::Sealed {
    /// This is set to the return type of the `Future`.
    type Output;

    /// Put the thread to sleep until the `Future` is ready to return a value.
    fn wait(self) -> Self::Output
    where
        Self: Sized;
}

impl<F> sealed::Sealed for F where F: Future {}

#[cfg_attr(feature = "std", allow(unused_mut))]
fn wait_block_on<F>(mut fut: F) -> F::Output
where
    F: Future + Sized,
{
    #[cfg(feature = "std")]
    let waker = Arc::new(ThreadWaker {
        thread: thread::current(),
    });
    #[cfg(not(feature = "std"))]
    let waker = {
        let raw_waker = RawWaker::new(core::ptr::null(), &VTABLE);
        #[allow(unsafe_code)]
        unsafe {
            Waker::from_raw(raw_waker)
        }
    };

    let waker = Waker::from(waker);
    let mut context = Context::from_waker(&waker);

    #[cfg(feature = "std")]
    let mut future = Box::pin(fut);

    #[cfg(not(feature = "std"))]
    #[allow(unsafe_code)]
    let mut future = unsafe { Pin::new_unchecked(&mut fut) };

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                #[cfg(feature = "std")]
                thread::park();
                #[cfg(not(feature = "std"))]
                for _ in 0..100 {
                    core::hint::spin_loop();
                }
            }
        }
    }
}

impl<F> Waitable for F
where
    F: Future,
{
    type Output = F::Output;

    fn wait(self) -> Self::Output
    where
        Self: Sized,
    {
        wait_block_on(self)
    }
}

mod sealed {
    pub trait Sealed {}
}

pub mod prelude {
    //! This is a convenience module that makes the magic happen.
    //!
    //! The alternative is to import the [`Waitable`] trait directly.
    //!
    //! # Example
    //! ```rust
    //! use wait::prelude::*;
    //!
    //! async fn add(a: usize, b: usize) -> usize {
    //!    a + b
    //! }
    //!
    //! fn main() {
    //!   let result = add(2, 2).wait();
    //!   assert_eq!(result, 4);
    //! }
    //! ```
    //!
    //! [`Waitable`]: super::Waitable

    pub use super::Waitable as _;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    async fn add(a: usize, b: usize) -> usize {
        a + b
    }

    async fn mul(a: usize, b: usize) -> usize {
        let mut result = 0;
        for _ in 0..a {
            result = add(result, b).await;
        }
        result
    }

    #[test]
    fn test_single_level() {
        let result = add(2, 2).wait();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sequential_calls() {
        let result1 = add(1, 2).wait();
        let result2 = add(2, 3).wait();

        assert_eq!(result1, 3);
        assert_eq!(result2, 5);
    }

    #[test]
    fn test_nested_calls() {
        let result = mul(2, 3).wait();

        assert_eq!(result, 6);
    }
}

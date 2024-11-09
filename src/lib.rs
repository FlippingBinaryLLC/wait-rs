#![no_std]
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://flippingbinary.com/wait-rs/favicon.ico")]

use core::future::Future;

#[cfg(not(feature = "tokio"))]
use core::task::{Context, Poll, Waker};

#[cfg(all(not(feature = "tokio"), not(feature = "std")))]
static VTABLE: core::task::RawWakerVTable = core::task::RawWakerVTable::new(
    |_| core::task::RawWaker::new(core::ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

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

#[cfg(all(not(feature = "tokio"), feature = "std"))]
fn std_wait_block_on<F>(fut: F) -> F::Output
where
    F: Future + Sized,
{
    extern crate alloc;
    extern crate std;

    use std::thread;

    use alloc::{boxed::Box, sync::Arc, task::Wake};

    struct ThreadWaker {
        thread: thread::Thread,
    }

    impl Wake for ThreadWaker {
        fn wake(self: Arc<Self>) {
            self.thread.unpark();
        }
    }

    let waker = Arc::new(ThreadWaker {
        thread: thread::current(),
    });

    let waker = Waker::from(waker);
    let mut context = Context::from_waker(&waker);

    let mut future = Box::pin(fut);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                thread::park();
            }
        }
    }
}

#[cfg(all(not(feature = "tokio"), not(feature = "std")))]
fn nostd_wait_block_on<F>(mut fut: F) -> F::Output
where
    F: Future + Sized,
{
    use core::{hint::spin_loop, pin::Pin, ptr::null, task::RawWaker};

    let waker = {
        let raw_waker = RawWaker::new(null(), &VTABLE);
        #[allow(unsafe_code)]
        unsafe {
            Waker::from_raw(raw_waker)
        }
    };

    #[allow(unsafe_code)]
    let mut future = unsafe { Pin::new_unchecked(&mut fut) };

    let mut context = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(result) => return result,
            Poll::Pending => {
                for _ in 0..100 {
                    spin_loop();
                }
            }
        }
    }
}

#[cfg(feature = "tokio")]
fn tokio_wait_block_on<F>(fut: F) -> F::Output
where
    F: Future + Sized,
{
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::CurrentThread {
            panic!("Cannot block on a future from within a CurrentThread runtime");
        } else {
            tokio::task::block_in_place(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(fut)
            })
        }
    } else {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut)
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
        #[cfg(all(not(feature = "tokio"), feature = "std"))]
        return std_wait_block_on(self);
        #[cfg(all(not(feature = "tokio"), not(feature = "std")))]
        return nostd_wait_block_on(self);
        #[cfg(feature = "tokio")]
        return tokio_wait_block_on(self);
    }
}

mod sealed {
    pub trait Sealed {}
}

pub mod prelude {
    //! This is the convenience module where the magic happens.
    //!
    //! The alternative is to import the [`Waitable`] trait directly.
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

    // Test the tokio runtime with reqwest only if tokio feature is enabled
    #[cfg(feature = "tokio")]
    #[test]
    fn test_on_future_that_requires_tokio() {
        let response = reqwest::get("https://www.rust-lang.org").wait().unwrap();
        assert!(response.status().is_success());
    }

    #[cfg(feature = "tokio")]
    #[test]
    #[should_panic]
    fn test_inside_single_thread_tokio_runtime() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();

        let _ = runtime
            .block_on(async { reqwest::get("https://www.rust-lang.org").wait() })
            .is_err();
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn test_inside_multi_thread_tokio_runtime_with_no_timers_or_io() {
        let response = tokio::runtime::Builder::new_multi_thread()
            .build()
            .unwrap()
            .block_on(async { reqwest::get("https://www.rust-lang.org").wait() })
            .unwrap();

        assert!(response.status().is_success());
    }
}

#![doc = include_str!("../README.md")]

use std::{
    future::Future,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    thread,
};

struct ThreadWaker {
    thread: std::thread::Thread,
}

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

impl<F> Waitable for F
where
    F: Future,
{
    type Output = F::Output;

    fn wait(self) -> Self::Output
    where
        Self: Sized,
    {
        let thread_waker = Arc::new(ThreadWaker {
            thread: thread::current(),
        });
        let waker = Waker::from(thread_waker);
        let mut context = Context::from_waker(&waker);
        let mut future = Box::pin(self);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(result) => return result,
                Poll::Pending => thread::park(),
            }
        }
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

#[doc(hidden)]
#[deprecated(since = "0.1.1", note = "use `wait::prelude::*` instead")]
pub mod preamble {
    #[doc(hidden)]
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

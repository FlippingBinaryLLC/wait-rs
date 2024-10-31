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

/// The `Waitable` trait defines a method that is implemented for all
/// [`Future`] types. This allows you to call the `.wait()` method on any
/// `Future` outside of an `async` context.
pub trait Waitable {
    type Output;

    fn wait(self) -> Self::Output
    where
        Self: Sized;
}

/// This is the implementation of the [`Waitable`] trait for every `Future`.
/// All `async` functions return a `Future`, so this attaches the `.wait()`
/// method to every `async` function.
///
/// All it does is put the thread to sleep until the [`Future`] is ready
/// to return a value.
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

pub mod preamble {
    //! This is a convenience module that re-exports the `Waitable` trait so that
    //! every `async` function can be resolved with a call to the `.wait()`
    //! method outside of an `async` context.
    //!
    //! # Example
    //! ```rust
    //! use wait::preamble::*;
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

    #[doc(hidden)]
    pub use super::Waitable as _;
}

#[cfg(test)]
mod tests {
    use super::preamble::*;

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

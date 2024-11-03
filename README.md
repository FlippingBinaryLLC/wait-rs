<h1 align="center">
  <a href="https://github.com/FlippingBinaryLLC/wait-rs"><img
    alt="A humanoid rests their chin in their hand, looking bored"
    src="https://flippingbinary.com/wait-rs/logo-square.png" width="400"></a>

Wait for Rust

</h1>

<p align="center">
<a
  href="https://github.com/FlippingBinaryLLC/wait-rs/actions?query=branch%3Amain"><img
    alt="Build Status"
    src="https://img.shields.io/github/actions/workflow/status/FlippingBinaryLLC/wait-rs/ci.yml?branch=main"></a>
<a
  href="https://crates.io/crates/wait"><img alt="Latest Release on crates.io"
  src="https://img.shields.io/crates/v/wait.svg"></a>
</p>

<p align="center">
<a href="https://docs.rs/wait">
  Documentation
</a>
</p>

**Wait for Rust** simplifies the integration of asynchronous code into
synchronous applications without the need to rewrite your entire application
as `async`.

## Problem Statement

In Rust, using `async` can be a double-edged sword. While it's powerful for
I/O-bound operations, it can introduce unnecessary complexity in CPU-bound
applications. You might want to use an external library that only provides
`async` functions without making your entire application asynchronous.

Common solutions include:

- Making the entire application `async`, which adds overhead and complexity.
- Using `block_on(async {})`, which adds boilerplate code and usually makes
  your dependency tree much larger.

Additionally, holding a `Mutex` lock across an `async` boundary can be
dangerous and lead to deadlocks or other concurrency issues.

## Solution

The **Wait for Rust** crate provides a simple and elegant solution. It allows
you to call `async` functions from synchronous contexts without coloring your
functions with `async`.

## Usage

Getting started with **Wait for Rust** is straightforward:

1. Either add the crate with `cargo add wait` or add it to your `Cargo.toml`
   manually:

   ```toml
   [dependencies]
   wait = "0.2"
   ```

   If you want to use an `async` function that explicitly requires the
   `tokio` runtime, you must enable the `tokio` feature or your code will
   panic at runtime. Enabling this feature brings in the minimum dependencies
   necessary to support running `tokio`-dependent code. Either add the
   feature while adding the crate with `cargo add wait --features tokio` or
   add it to your `Cargo.toml` manually:

   ```toml
   [dependencies]
   wait = { version = "0.2", features = ["tokio"] }
   ```

   **NOTE:** This is only necessary if your code panics with a message like
   `there is no reactor running, must be called from the context of a Tokio 1.x runtime`.

2. Use the `.wait()` method on any `async` function instead of `.await`:

   ```rust
   // The prelude attaches the `.wait()` method to all `async` functions
   use wait::prelude::*;

   // Define an `async` function or use one from an external library
   async fn add(a: i32, b: i32) -> i32 {
       a + b
   }

   fn main() {
       // Call the `async` function using .wait()
       let val = add(2, 2).wait();
       println!("Result: {}", val);
   }
   ```

3. ????

4. Profit

## Building with `no_std`

This crate is `no_std` compatible. To use it in a `no_std` environment,
disable the default features in your `Cargo.toml`:

```toml
[dependencies]
wait = { version = "0.2", default-features = false }
```

This is not an ideal solution because it has to busy wait. The crate attempts
to reduce energy consumption by letting the CPU know that it is in a busy
loop, but it is still not as efficient as a proper `async` runtime can be.

## Is This Library Necessary?

While you might not need this library for every project, it provides a
convenient way to integrate `async` functions into synchronous code. This
crate has no dependencies, so it won't add significantly to your build time.
It also reduces the boilerplate and complexity that often comes with other
solutions.

## Acknowledgements

This crate is built on the shoulders of giants. Rust futures are complicated,
but popular libraries like `tokio`, `async-std`, `futures-rs`, and `embassy`
are incredible resources for learning how futures work. We thank the
maintainers and contributors of these libraries and the broader Rust
community for all of their hard work and dedication. Additionally, the CI
workflow for this repository is heavily based on the one in the `futures-rs`
repository.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

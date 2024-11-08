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
  -
<a href="https://github.com/FlippingBinaryLLC/wait-rs">
  Website
</a>
</p>

**Wait for Rust** simplifies the integration of asynchronous code into
synchronous applications without the need to rewrite your entire application
as `async` or add a bulky runtime just to use `block_on()`.

## Installation

Either use `cargo add wait` or add it to your `Cargo.toml` manually. You can
also add the `tokio` feature if your `async` function requires it (like
`reqwest` does, for example).

## Usage

Getting started with **Wait for Rust** is straightforward:

**Step 1**: Add the prelude to your application:

```rust
use wait::prelude::*;
```

**Step 2**: Call the `.wait()` method instead of `.await` even though your
function is not also `async`:

```rust
let body = reqwest::get("https://www.rust-lang.org")
  .wait()?
  .text()
  .wait()?;

println!("body = {body:?}");
```

**Step 3**: ????

**Step 4**: Profit

You can see the complete example in the [`examples`] folder.

## Building with `no_std`

This crate is `no_std` by default with libraries from `std` and `alloc` only
pulled in when the `std` feature flag is enabled (which it is by default).
The `tokio` feature flag brings in a `tokio` runtime, which also requires
`std`.

Without the `std` feature flag, this library uses a hot loop to wait for the
`Future` to complete. This is not ideal, so it should only be used when
absolutely necessary.

## Troubleshooting

If your application panics with the message `there is no reactor running,
must be called from the context of a Tokio 1.x runtime`, you can fix this by
enabling the `tokio` feature flag in your crate.

If you encounter any other problems, please [open an issue] on GitHub.

## Acknowledgements

This crate is built on the shoulders of giants. Rust futures are complicated,
but popular libraries like `tokio`, `async-std`, `futures-rs`, and `embassy`
are incredible resources for learning how futures work. We thank the
maintainers and contributors of these libraries and the broader Rust
community for all of their hard work and dedication. Additionally, the CI
workflow for this repository is heavily based on the one in the `futures-rs`
repository.

## License

Licensed under either of the [Apache License, Version 2.0][APACHE-2.0] or the
[MIT license][MIT] at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[`examples`]: https://github.com/FlippingBinaryLLC/wait-rs/tree/main/examples
[open an issue]: https://github.com/FlippingBinaryLLC/wait-rs/issues
[APACHE-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT]: https://opensource.org/licenses/MIT

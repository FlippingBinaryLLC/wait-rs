# Wait For Rust

The classic problem with `async` is the way that it colors every function it
touches. Maybe your application is not I/O bound, so you don't want to make
it `async`, but you want to use some library that happens to only export
`async` functions.

What do you do?

You could make your application fully `async`, but that seems like overkill
for an application that is cpu-bound. You could call `block_on(async {})`
from an `async` library, but that seems like a lot of overhead and
boilerplate just to avoid coloring your functions.

On top of that, it can be dangerous to hold a `Mutex` lock across an `async`
boundary. This can be frustrating when you have a `Mutex` that you want to
hold for the duration of a function scope, but you need to call a function in
an external library with only `async` functions.

That's where this library comes in. It provides a little syntactic sugar that
lets you easily wait on the results of an `async` function without coloring
your functions with `async`.

Is this library necessary? Probably not. But this is more convenient than
some alternatives.

## Usage

It's as easy as 1, 2, 3.

**Step 1**: Add this to your `Cargo.toml`:

```toml
[dependencies]
wait = "0.1"
```

**Step 2**: Add the prelude to the top of any file where you want to use it:

```rust
use wait::prelude::*;
```

**Step 3**: Use the `.wait()` method on any `async` function instead of `.await`.

## Example

```rust
use wait::prelude::*;

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let val = add(2, 2).wait();
    println!("Result: {}", val);
}
```

## License

This library is distributed under the terms of either the
[MIT](https://opensource.org/licenses/MIT) license, or the
[Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) license, at your
option.

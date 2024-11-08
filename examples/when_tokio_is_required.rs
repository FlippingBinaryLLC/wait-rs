//! This is a simple example from the `reqwest` crate rewritten to use the
//! `.wait()` syntax.

use wait::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This won't compile because `main` is not an `async` function:

    // let body = reqwest::get("https://www.rust-lang.org")
    //     .await?
    //     .text()
    //     .await?;

    // ... But this will:

    let body = reqwest::get("https://www.rust-lang.org")
        .wait()?
        .text()
        .wait()?;

    println!("body = {body:?}");

    Ok(())
}

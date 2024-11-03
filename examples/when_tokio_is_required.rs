//! This is a simple example from the `reqwest` crate rewritten to use the
//! `.wait()` syntax.

use wait::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .wait()?
        .text()
        .wait()?;

    println!("body = {body:?}");

    Ok(())
}

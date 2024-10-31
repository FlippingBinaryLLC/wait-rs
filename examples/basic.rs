use wait::preamble::*;

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let val = add(2, 2).wait();
    println!("Result: {}", val);
}

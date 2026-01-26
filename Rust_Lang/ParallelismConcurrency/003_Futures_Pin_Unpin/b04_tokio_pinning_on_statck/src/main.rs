use std::pin::pin;

async fn example() {
    let s = String::from("I want to stay here");

    // pin! macro (nightly) or pin_utils crate
    let pinned = pin!(s);

    println!("pinned {}", pinned);

    // Now we can create self-referential futures / iterators that point into `s`
    // e.g. streaming parser, generator, etc.

    // pinned.as_mut().do_something_that_keeps_reference();
}

#[tokio::main]
async fn main() {
    example().await;
}

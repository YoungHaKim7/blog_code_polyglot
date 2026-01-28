use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();

    for word in ["a", "b", "a", "c", "b", "a"] {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("word count : {counts:?}");
}

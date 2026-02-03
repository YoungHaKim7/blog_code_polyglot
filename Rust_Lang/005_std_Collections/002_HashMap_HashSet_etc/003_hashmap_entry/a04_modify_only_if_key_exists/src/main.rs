use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("x", 10);

    println!("insert map {map:?}");
    map.entry("x").and_modify(|v| *v += 5);
    map.entry("y").and_modify(|v| *v += 5); // nothing happens
    println!("map {map:?}");
}

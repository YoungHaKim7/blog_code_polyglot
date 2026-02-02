use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.entry("expensive").or_insert_with(|| {
        println!("computed!");
        42
    });

    println!("{:?}", map);
}

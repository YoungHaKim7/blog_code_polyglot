use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();

    map.entry("nums").or_default().push(10);
    map.entry("nums").or_default().push(20);
    println!("map :  {map:?}");

    let mut map02: HashMap<&str, Option<u32>> = HashMap::new();
    map02.entry("poneyland").or_default();

    assert_eq!(map02["poneyland"], None);
}

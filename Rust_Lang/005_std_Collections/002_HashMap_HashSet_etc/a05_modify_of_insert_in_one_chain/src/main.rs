use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.entry("count").and_modify(|v| *v += 1).or_insert(1);
    println!("map : {map:?}");
    map.entry("count").and_modify(|v| *v += 2).or_insert(1);
    map.entry("count").and_modify(|v| *v += 10).or_insert(1);
    println!("map : {map:?}");

    let mut map02: HashMap<&str, u32> = HashMap::new();
    map02.entry("poneyland").or_insert(3);
    assert_eq!(map02["poneyland"], 3);
    *map02.entry("poneyland").or_insert(10) *= 2;
    assert_eq!(map02["poneyland"], 6);
}

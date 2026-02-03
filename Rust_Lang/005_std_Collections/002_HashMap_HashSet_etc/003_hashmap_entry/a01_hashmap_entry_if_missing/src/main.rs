use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.entry("apple").or_insert(3);
    map.entry("apple").or_insert(10); // ignored

    assert_eq!(map["apple"], 3);
}

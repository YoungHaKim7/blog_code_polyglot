use std::collections::HashMap;

fn main() {
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for word in ["apple", "ant", "banana"] {
        let first = word.chars().next().unwrap();
        groups.entry(first).or_default().push(word);
    }

    println!("{groups:?}");
}

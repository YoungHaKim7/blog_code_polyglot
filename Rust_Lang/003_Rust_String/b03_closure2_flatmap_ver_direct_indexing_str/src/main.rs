fn main() {
    std::env::args()
        .skip(1)
        .flat_map(|c| c.chars().collect::<Vec<_>>().into_iter().enumerate())
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(i, c)| println!("arg[{}] = {}", i, c));
}

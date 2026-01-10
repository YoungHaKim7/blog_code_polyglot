fn main() {
    for arg in std::env::args().skip(1) {
        let scalars: Vec<char> = arg.chars().collect();
        for i in 0..arg.len() {
            println!("arg[{}] = {}", i, scalars[i])
        }
    }
}

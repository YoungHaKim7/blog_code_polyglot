fn main() {
    for (i, arg) in std::env::args().skip(1).enumerate() {
        println!("arg[{}] = {}", i, arg);
    }
}

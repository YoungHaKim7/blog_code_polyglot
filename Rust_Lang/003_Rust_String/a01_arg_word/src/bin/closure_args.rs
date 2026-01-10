fn main() {
    std::env::args().skip(1).enumerate().for_each(|(i, arg)| {
        println!("arg[{}] = {}", i, arg);
    });
}

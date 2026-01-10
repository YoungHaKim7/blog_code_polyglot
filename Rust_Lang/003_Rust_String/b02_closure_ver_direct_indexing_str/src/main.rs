fn main() {
    std::env::args().skip(1).for_each(|arg| {
        arg.chars().enumerate().for_each(|(i, c)| {
            println!("arg[{}] = {}", i, c);
        });
    });
}

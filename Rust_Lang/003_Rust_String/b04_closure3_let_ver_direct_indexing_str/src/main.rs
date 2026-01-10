fn main() {
    std::env::args().skip(1).for_each(|arg| {
        let scalars: Vec<char> = arg.chars().collect();
        scalars.iter().enumerate().for_each(|(i, c)| {
            println!("arg[{}] = {}", i, c);
        });
    });
}

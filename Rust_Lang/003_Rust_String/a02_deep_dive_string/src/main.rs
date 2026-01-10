fn main() {
    let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");

    println!("upp = {}", arg.to_uppercase());
    println!("UPP = {}", arg);
}

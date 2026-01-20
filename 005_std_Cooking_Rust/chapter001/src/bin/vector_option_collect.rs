fn main() {
    let alphabet: Vec<_> = (b'A'..b'z' + 1)
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect();

    println!("{alphabet:?}");
}

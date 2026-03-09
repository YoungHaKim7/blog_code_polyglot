fn bin4(x: u8) -> String {
    format!("{:04b}", x)
}

fn main() {
    let x = 11; // 1011
    //       x >> 1 // 0101
    //  x ^ (x >>1) // 1110
    let gray = x ^ (x >> 1);

    println!("x    : {}", bin4(x));
    println!("gray : {}", bin4(gray));
}

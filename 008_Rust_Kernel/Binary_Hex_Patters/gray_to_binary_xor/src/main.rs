fn gray_to_binary(mut b: u32) -> u32 {
    b ^= b >> 1;
    b ^= b >> 2;
    b ^= b >> 4;
    b ^= b >> 8;
    b ^= b >> 16;
    b
}

fn main() {
    let gray: u32 = 0b1110; // Gray code
    let binary = gray_to_binary(gray);

    println!("gray   : {:04b}", gray);
    println!("binary : {:04b}", binary);
}

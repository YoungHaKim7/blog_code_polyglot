fn gray_to_binary(mut b: u32) -> u32 {
    b ^= b >> 1;
    b ^= b >> 2;
    b ^= b >> 4;
    b ^= b >> 8;
    b ^= b >> 16;
    b
}

fn gray_to_binary02(mut x: u32) -> u32 {
    let mut shift = 1;
    while shift < 32 {
        x ^= x >> shift;
        shift <<= 1;
    }
    x
}

fn main() {
    let gray: u32 = 0b1110; // Gray code
    let binary = gray_to_binary(gray);
    let binary02 = gray_to_binary02(gray);

    println!("gray   : {:04b}", gray);
    println!("binary : {:04b}", binary);
    println!("binary : {:04b}", binary02);
}

// (a - 1) // 7  , a = 8
// 8   0000 1000
// 7   0000 0111
// 13  0000 1101
// +   0001 0100  (20dec)

//                       0001 0100  (20dec)
// !(a - 1) // 248u8   0b1111 1000
//                       0001 0000  (16dec)  align 16

fn align_up(x: usize, a: usize) -> usize {
    (x + (a - 1)) & !(a - 1)
}

fn main() {
    let x = 13;
    let a = 8;

    let aligned = align_up(x, a);

    let negative_a_minus = !(a - 1);

    println!("x       : {}", x);
    println!("aligned : {}", aligned);

    println!(" ~~~~");
    println!("negative!(8 -1) : {}", negative_a_minus as u8);
    println!("negative!(8 -1) : {:08b}", negative_a_minus as u8);
}

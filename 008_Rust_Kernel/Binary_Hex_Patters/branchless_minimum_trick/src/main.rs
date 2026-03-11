// 10   // 0000 1010
// 6    // 0000 0110
// x ^ y // 0000 1100    // 10 dec

fn branchless_min(x: i32, y: i32) -> i32 {
    y ^ ((x ^ y) & -((x < y) as i32))
}

fn main() {
    let x = 10;
    let y = 6;

    let min = branchless_min(x, y);

    println!("x   : {}", x);
    println!("y   : {}", y);
    println!("min : {}", min);

    let test_eval = x < y as i32;
    println!("10 < 6 as i32 : {}", test_eval);
    println!("-(10 < 6 as i32) : {}", -(test_eval as i32));
    // println!("10 < 6 as i32 : {:08b}", test_eval);
}

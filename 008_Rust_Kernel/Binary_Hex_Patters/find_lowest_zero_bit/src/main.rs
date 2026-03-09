fn main() {
    let x = 47; // 0010 1111
    // !x            // 1101 0000
    //&(x+1) dec 48  // 0011 0000
    //       dec 16  // 0001 0000
    let res = !x & (x + 1);
    println!("{res}");
}

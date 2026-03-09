fn main() {
    let x = 47; // 0010 1111
    //&(x-1) dec 46  // 0010 1110
    //       dec 46  // 0010 1110
    let res = (x & (x - 1)) == 0;
    println!("{res}");
}


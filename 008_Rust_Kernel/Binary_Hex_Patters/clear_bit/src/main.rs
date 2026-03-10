fn main() {
    let negative_x = !(1 << 2);
    let n = 2; //   0000 0010
    let x = 45; //  0010 1101
    //  1            //  0000 0001
    // (1 << 2) 4dec //  0000 0100
    //\!(1 << 2)-5dec//  1111 1101
    //&!(1 << 2)     //  0000 1011
    //  res  41 dec  //  0010 1001
    let res = x & !(1 << n);
    println!("{negative_x}");
    println!("negative x binariy : {:8b}", negative_x as u8);
    println!("{res}");
}

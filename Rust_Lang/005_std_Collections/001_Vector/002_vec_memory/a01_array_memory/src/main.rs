fn main() {
    // 0 , 1, 2, 3
    let arr = [1, 2, 3, 4, 5];
    let arr02 = [1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 13];
    println!("arr []: memory address : {:p}", &arr);
    for (i, val) in arr.iter().enumerate() {
        println!("arr[{}]: value={}, memory address : {:p}", i, val, val);
    }

    let my_vec: Vec<i32> = Vec::new();
    let my_vec02 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let my_vec03 = vec![0; 10];
    println!("{my_vec03:?}");
    println!("vec {} bytes", size_of_val(&my_vec03));

    println!("my_vec02 32 bytes? : {}", size_of_val(&my_vec02));
    println!("arr : {} bytes", size_of_val(&arr));
    println!("arr02 : {} bytes", size_of_val(&arr02));

    println!("my_vec 02 adress : {:p}", &my_vec02);

    for (i, val) in my_vec02.iter().enumerate() {
        println!("my_vec 02 [{}] adress : {:p}", i, &val);
    }

    println!("my_vec 02 [{}]: {:p}", 0, &my_vec02[0]);
    println!("my_vec 02 [{}]: {:p}", 1, &my_vec02[1]);
}

fn squares(x: i32) -> i32 {
    x * x
}

fn main() {
    let my_arr = [10, 20, 30, 40];
    let my_idx_2 = my_arr[2];
    let my_arr_map_2 = my_arr.map(|c| c * 3);
    let my_squares = my_arr.map(|c| c * c);
    let my_squares_2 = my_arr.map(squares);
    let my_is_even = my_arr.iter().filter(|c| *c % 2 == 0);
    println!("basic list : {my_arr:?}");
    println!("arr idx 2 : {my_idx_2:?}");
    println!("arr map :{my_arr_map_2:?}");
    println!("arr squares :{my_squares:?}");
    println!("arr squares :{my_squares_2:?}");
    println!("arr filter(isEven) :{my_is_even:?}");
    print!("arr filter(isEven) : [");
    for i in my_is_even {
        if len() - 1 {
            print!("{}, ", i);
        } else {
            print! {"{}", i};
        }
    }
    println!("]");
}

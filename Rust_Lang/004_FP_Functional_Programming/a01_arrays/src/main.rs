fn squares(x: i32) -> i32 {
    x * x
}

fn main() {
    let my_arr = [10, 20, 30, 40];
    println!("basic list : {my_arr:?}");

    let my_idx_2 = my_arr[2];
    println!("arr idx 2 : {my_idx_2:?}");

    let my_arr_map_2 = my_arr.map(|c| c * 3);
    println!("arr map :{my_arr_map_2:?}");

    let my_squares = my_arr.map(|c| c * c);
    println!("arr squares :{my_squares:?}");

    let my_squares_2 = my_arr.map(squares);
    println!("arr squares :{my_squares_2:?}");

    let my_is_even = my_arr.iter().filter(|c| *c % 2 == 0);
    println!("arr filter(isEven) :{my_is_even:?}");
    print!("arr filter(isEven) : [");
    for (idx, i) in my_is_even.enumerate() {
        if idx > 0 {
            print!(", ",);
        }
        print!("{}", i);
    }
    println!("]");

    let my_arr_filter25: Vec<i32> = my_arr
        .iter()
        .filter_map(|c| if c > &25 { Some(*c) } else { None })
        .collect();
    println!("arr filter > 25 :{my_arr_filter25:?}");
}

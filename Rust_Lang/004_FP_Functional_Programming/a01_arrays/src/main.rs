fn my_squares(x: i32) -> i32 {
    x * x
}

trait MySumExt: Sized {
    fn my_sum(self) -> i32;
    fn my_product(self) -> i32;
}

// impl<T: Iterator<Item = i32>> MySumExt for T
impl<T> MySumExt for T
where
    T: Iterator<Item = i32>,
{
    fn my_sum(self) -> i32 {
        let mut total = 0;
        for num in self {
            total += num;
        }
        total
    }

    fn my_product(self) -> i32 {
        let mut my_total = 1;
        for num in self {
            my_total *= num;
        }
        my_total
    }
}

fn main() {
    let my_arr = [10, 20, 30, 40];
    println!("basic list : {my_arr:?}");

    let my_idx_2 = my_arr[2];
    println!("arr idx 2 : {my_idx_2:?}");

    let my_arr_map_2 = my_arr.map(|c| c * 3);
    println!("arr map :{my_arr_map_2:?}");

    let squares_arr = my_arr.map(|c| c * c);
    println!("arr squares :{squares_arr:?}");

    let my_squares_2 = my_arr.map(my_squares);
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

    // my_sum()  custom method
    let sum_arr: i32 = my_arr.iter().copied().my_sum();
    println!("arr sum :{sum_arr:?}");

    // std sum() method
    let sum_arr02: i32 = my_arr.iter().sum();
    println!("arr sum :{sum_arr02:?}");

    // Fold : Reduce to single value
    let product_arr = my_arr.iter().copied().my_product();
    println!("arr product :{product_arr:?}");
}

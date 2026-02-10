fn debug_vec(v: &Vec<i32>) {
    println!("len = {}", v.len());
    println!("cap = {}", v.capacity());
    println!("ptr = {:p}", v.as_ptr());

    for i in 0..v.len() {
        unsafe {
            println!(
                "  [{}] addr = {:p}, value = {}",
                i,
                v.as_ptr().add(i),
                *v.as_ptr().add(i)
            );
        }
    }

    // for (i, val) in v.iter().enumerate() {
    //     unsafe {
    //         println!(
    //             "  [{}] addr = {:p}, value = {}",
    //             i,
    //             v.as_ptr().add(i),
    //             *v.as_ptr().add(i)
    //         );
    //     }
    // }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    println!("== After Vec::new() ==");
    debug_vec(&v);

    v.push(10);
    println!("\n== After push(10) ==");
    debug_vec(&v);

    v.push(20);
    println!("\n== After push(20) ==");
    debug_vec(&v);

    v.push(30);
    println!("\n== After push(30) ==");
    debug_vec(&v);
}

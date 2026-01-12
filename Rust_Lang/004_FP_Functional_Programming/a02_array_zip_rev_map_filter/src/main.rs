fn main() {
    // === Basic List ===
    let numbers = vec![10, 20, 30, 40];

    println!("=== Basic List ===");
    println!("{numbers:?}"); // [10, 20, 30, 40]

    // === Index Access ===
    let value = numbers[2];
    println!("\n=== Index Access ===");
    println!("{value}"); // 30

    // === Map: Transform each element ===
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    let times_three: Vec<i32> = numbers.iter().map(|x| x * 3).collect();

    println!("\n=== Map: Transform each element ===");
    println!("{squares:?}");
    println!("{times_three:?}");

    // === Filter: Select elements ===
    let evens: Vec<i32> = numbers.iter().copied().filter(|x| x % 2 == 0).collect();
    let greater_than_25: Vec<i32> = numbers.iter().copied().filter(|x| *x > 25).collect();

    println!("\n=== Filter: Select elements ===");
    println!("{evens:?}");
    println!("{greater_than_25:?}");

    // === Fold: Reduce to single value ===
    let total: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    let foldl = numbers.iter().fold(0, |acc, x| acc + x);
    let foldr = numbers.iter().rev().fold(0, |acc, x| x - acc);

    println!("\n=== Fold: Reduce to single value ===");
    println!("{total}");
    println!("{product}");
    println!("{foldl}");
    println!("{foldr}");

    // === List Operations ===
    println!("\n=== List Operations ===");
    println!("{:?}", numbers.first()); // Some(10)
    println!("{:?}", &numbers[1..]); // [20, 30, 40]
    println!("{:?}", &numbers[..numbers.len() - 1]); // [10, 20, 30]
    println!("{:?}", numbers.last()); // Some(40)
    println!("{}", numbers.len());
    println!("{}", numbers.is_empty());

    // === List Construction ===
    println!("\n=== List Construction ===");
    let mut cons = vec![5];
    cons.extend(&numbers);
    println!("{cons:?}");

    let mut appended = numbers.clone();
    appended.extend([50, 60]);
    println!("{appended:?}");

    println!("{:?}", vec![7; 4]); // replicate
    println!("{:?}", &numbers[..3]); // take
    println!("{:?}", &numbers[2..]); // drop

    // === List Comprehensions ===
    println!("\n=== List Comprehensions ===");
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let filtered: Vec<i32> = numbers.iter().copied().filter(|x| *x > 25).collect();

    let cartesian: Vec<(i32, i32)> = (1..=3)
        .flat_map(|x| (10..=12).map(move |y| (x, y)))
        .collect();

    println!("{doubled:?}");
    println!("{filtered:?}");
    println!("{cartesian:?}");

    // === Zipping ===
    println!("\n=== Zipping ===");
    let zipped: Vec<(i32, char)> = (1..=4).zip("abcd".chars()).collect();
    let zip_with: Vec<i32> = numbers.iter().zip(1..=4).map(|(a, b)| a + b).collect();

    println!("{zipped:?}");
    println!("{zip_with:?}");

    // === More Useful Functions(rev) ===
    println!("\n=== More Useful Functions ===");
    let reversed: Vec<i32> = numbers.iter().rev().copied().collect();
    let min = numbers.iter().min();
    let max = numbers.iter().max();

    println!("{reversed:?}");
    println!("{min:?}");
    println!("{max:?}");
}

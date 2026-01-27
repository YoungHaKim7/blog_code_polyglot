// https://doc.rust-lang.org/std/fmt/

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler_ = dhat::Profiler::new_heap();
    let my_num = format!("나의 좋아하는 숫자는 : {}", 42);
    println!("출력 해보자 : {my_num}");
    println!("{:p}", &my_num);
    println!("{}", size_of_val(&my_num));

    let color = "black";

    let hello = "hello ";
    let world = "world ";

    println!("내가 좋아하는 색깔은 {}, {}, {}", color, hello, world);

    let greeting = "Hello";

    let a = 10;
    let b = 10.2222222;
    let formatted = format!("Interger: {}, Float: {:.2}", a, b);
    println!("a b formatted : {formatted}");

    println!(" a b {0} {1}  \n\nafter{1} {0}", b, a);
    println!("{}", format!("Hello")); // => "Hello"
    println!("{}", format!("Hello, {}!", "world")); // => "Hello, world!"
    println!("{}", format!("The number is {}", 1)); // => "The number is 1"
    println!("{}", format!("{:?}", (3, 4))); // => "(3, 4)"
    println!("{}", format!("{value}", value = 4)); // => "4"
    let people = "Rustaceans";
    println!("{}", format!("Hello {people}!")); // => "Hello Rustaceans!"
    println!("{}", format!("{} {}", 1, 2)); // => "1 2"
    println!("{}", format!("{:04}", 42)); // => "0042" with leading zeros
    println!("{}", format!("{:#?}", (100, 200))); // => "(
    //       100,
    //       200,
    //     )"
}

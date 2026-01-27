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

    let _greeting = "Hello";
}

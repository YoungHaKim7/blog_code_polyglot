use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct MyNum(i32);

impl Add for MyNum {
    type Output = MyNum;

    fn add(self, rhs: MyNum) -> MyNum {
        MyNum(self.0 + rhs.0)
    }
}
//
// associated type
fn sum<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Add<Output = I::Item>,
{
    iter.reduce(|a, b| a + b)
}

fn main() {
    let nums = vec![MyNum(1), MyNum(2), MyNum(3)];
    let result = sum(nums.into_iter());

    println!("{:?}", result); // Some(MyNum(6))
}

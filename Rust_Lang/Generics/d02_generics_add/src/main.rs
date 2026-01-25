// fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T
fn add<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}
fn main() {
    add(2, 2);
    add(2.666, 3.001);
}

use std::ops::Rem;

fn for_some<T>(x: &[T]) -> bool
where
    T: Copy + Rem<T, Output = T> + PartialEq<T> + From<u8>,
{
    if x.iter().any(|xi| *xi % T::from(2) == T::from(0)) {
        true
    } else {
        false
    }
}

fn for_all<T>(x: &[T]) -> bool
where
    T: Copy + Rem<T, Output = T> + PartialEq<T> + From<u8>,
{
    if x.iter().all(|xi| *xi % T::from(2) == T::from(0)) {
        true
    } else {
        false
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![1, 2, 3, 4, 5];
    let is_some = for_some(&a);
    println!("a is some : {is_some}");
    let is_all = for_all(&b);
    println!("b is all : {is_all}");
}

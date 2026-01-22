fn first<I, T>(iter: I) -> Option<T>
where
    I: IntoIterator<Item = T>,
{
    iter.into_iter().next()
}

fn main() {
    let x = first(vec![10, 20, 30]);
    assert_eq!(x, Some(10));
}

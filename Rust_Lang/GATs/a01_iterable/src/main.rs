// Before GATs: Cannot express "Item depends on lifetime of &self"
// This is what we WANTED to write (but couldn't before GATs):
// trait Iterable {
//     type Item<'a>;  // ❌ Error: associated type cannot have lifetime parameters
//     fn iter<'a>(&'a self) -> Self::Item<'a>;
// }

// With GATs (Rust 1.65+): We can finally write natural APIs!
trait IterableGAT {
    type Item<'a> where Self: 'a;

    fn iter<'a>(&'a self) -> Self::Item<'a>;
}

struct MyVec<T>(Vec<T>);

impl<T> IterableGAT for MyVec<T> {
    type Item<'a> = std::slice::Iter<'a, T> where Self: 'a;

    fn iter<'a>(&'a self) -> Self::Item<'a> {
        self.0.iter()
    }
}

fn main() {
    let v = MyVec(vec![1, 2, 3]);

    // Zero-copy iteration over borrowed data
    for item in v.iter() {
        println!("{}", item);
    }
}

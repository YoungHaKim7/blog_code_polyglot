trait BorrowingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// A simple slice iterator that borrows from the underlying data
struct SliceIter<'slice, T> {
    data: &'slice [T],
    index: usize,
}

impl<'slice, T> BorrowingIterator for SliceIter<'slice, T> {
    type Item<'a> = &'a T where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

fn main() {
    let data = vec
![1, 2, 3, 4, 5];

    // Create iterator borrowing from data
    let mut iter = SliceIter { data: &data, index: 0 };

    // Zero-copy iteration: items are borrowed, not cloned
    while let Some(item) = iter.next() {
        println!("Item: {}", item);
    }

    println!("\nOriginal data still valid: {:?}", data);
}

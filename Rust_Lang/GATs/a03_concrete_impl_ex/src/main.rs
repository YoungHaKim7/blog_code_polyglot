struct SliceIter<'s, T> {
    slice: &'s [T],
    pos: usize,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

trait MyIter {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

impl<'s, T> MyIter for SliceIter<'s, T> {
    type Item<'a>
        = &'a T
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        let item = self.slice.get(self.pos)?;
        self.pos += 1;
        Some(item)
    }
}

fn main() {
    // Example 1: Basic iteration with integers
    println!("=== Example 1: Basic integer iteration ===");
    let numbers = vec![10, 20, 30, 40, 50];
    let mut iter = SliceIter {
        slice: &numbers,
        pos: 0,
    };
    while let Some(n) = iter.next() {
        println!("Got: {}", n);
    }

    // Example 2: Iteration with strings
    println!("\n=== Example 2: String iteration ===");
    let words = vec!["hello", "world", "GATs", "Rust"];
    let mut iter = SliceIter {
        slice: &words,
        pos: 0,
    };
    while let Some(word) = iter.next() {
        println!("Word: {}", word);
    }

    // Example 3: Multiple iterators from same data (zero-copy)
    println!("\n=== Example 3: Multiple iterators ===");
    let data = vec![1, 2, 3, 4, 5];
    let mut iter1 = SliceIter {
        slice: &data,
        pos: 0,
    };
    let mut iter2 = SliceIter {
        slice: &data,
        pos: 0,
    };
    println!("First iterator: {:?}", iter1.next());
    println!("Second iterator: {:?}", iter2.next());
    println!("First iterator again: {:?}", iter1.next());

    // Example 4: Iteration with custom types
    println!("\n=== Example 4: Custom type iteration ===");

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
    ];

    let mut iter = SliceIter {
        slice: &points,
        pos: 0,
    };

    while let Some(p) = iter.next() {
        println!("Point: {:?}", p);
    }

    // Example 5: Partial iteration and resume
    println!("\n=== Example 5: Partial iteration ===");
    let values = vec![100, 200, 300, 400, 500];
    let mut iter = SliceIter {
        slice: &values,
        pos: 0,
    };
    println!("First 2 items:");
    println!("  {:?}", iter.next());
    println!("  {:?}", iter.next());
    println!("Resume after break:");
    while let Some(v) = iter.next() {
        println!("  {:?}", v);
    }
}

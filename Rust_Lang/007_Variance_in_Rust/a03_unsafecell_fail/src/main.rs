use std::cell::UnsafeCell;

// Demonstrating invariance: Box<T> is covariant, but UnsafeCell<T> is invariant

// This works - Box is covariant, so Box<&'short i32> can become Box<&'long i32>
fn box_covariant_demo<'a, 'b>(x: Box<&'a i32>) -> Box<&'b i32>
where
    'a: 'b, // 'a outlives 'b, so &'a can shrink to &'b
{
    x // ✓ Covariant: shorter lifetime is OK
}

// This would NOT work - UnsafeCell is invariant
// fn unsafecell_invariant_demo<'a, 'b>(x: UnsafeCell<&'a i32>) -> UnsafeCell<&'b i32>
// where
//     'a: 'b,
// {
//     x  // ❌ ERROR: UnsafeCell is invariant, T cannot change
// }

fn main() {
    let value = 5;
    let cell = UnsafeCell::new(&value);

    // ❌ ERROR: cannot assign to immutable reference
    // *cell.get() = &value;

    // Demonstrate why invariance matters:
    // If UnsafeCell were covariant, this would compile and be unsound:

    /*
    fn evil<'long>(cell: UnsafeCell<&'long i32>) {
        // Suppose we could extend &'short to &'long here...
        // Then we could store a reference that outlives its data
        // via interior mutation!
    }
    */

    // Contrast with covariant types:
    let boxed: Box<&i32> = Box::new(&value);
    let _shrunk: Box<&'static i32> = box_covariant_demo(boxed);
    // This works because Box is covariant - we can shrink the lifetime

    // But UnsafeCell forbids this:
    // let _shrunk_unsafe: UnsafeCell<&'static i32> = unsafecell_invariant_demo(cell);
    // ❌ Would error: expected UnsafeCell<&'static i32>, found UnsafeCell<&value i32>
}

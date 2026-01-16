// Trait demonstrating GATs (Generic Associated Types) for HKT-like patterns
trait Container {
    // The GAT: View<'a> represents a borrowed view into the container
    type View<'a>
    where
        Self: 'a;

    fn view<'a>(&'a self) -> Self::View<'a>;
}

// Example 1: Vec - returns a slice view
impl<T> Container for Vec<T> {
    type View<'a> = &'a [T] where T: 'a;

    fn view<'a>(&'a self) -> &'a [T] {
        self.as_slice()
    }
}

// Example 2: Box - returns a reference to its contents
impl<T> Container for Box<T> {
    type View<'a> = &'a T where T: 'a;

    fn view<'a>(&'a self) -> &'a T {
        &**self
    }
}

// Example 3: Option - returns an Option with a reference
impl<T> Container for Option<T> {
    type View<'a> = Option<&'a T> where T: 'a;

    fn view<'a>(&'a self) -> Option<&'a T> {
        self.as_ref()
    }
}

// Example 4: Result - returns Result with references to both Ok and Err
impl<T, E> Container for Result<T, E> {
    type View<'a> = Result<&'a T, &'a E> where T: 'a, E: 'a;

    fn view<'a>(&'a self) -> Result<&'a T, &'a E> {
        self.as_ref()
    }
}

// Example 5: Array - returns a slice view
impl<T, const N: usize> Container for [T; N] {
    type View<'a> = &'a [T] where T: 'a;

    fn view<'a>(&'a self) -> &'a [T] {
        self.as_slice()
    }
}

fn main() {
    // Example 1: Vec container
    let vec_container: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec_view: &[i32] = vec_container.view();
    println!("Example 1 (Vec): container = {:?}, view = {:?}", vec_container, vec_view);

    // Example 2: Box container
    let box_container: Box<i32> = Box::new(42);
    let box_view: &i32 = box_container.view();
    println!("Example 2 (Box): container = {}, view = {}", box_container, box_view);

    // Example 3: Option container
    let option_container: Option<i32> = Some(100);
    let option_view: Option<&i32> = option_container.view();
    println!("Example 3 (Option): container = {:?}, view = {:?}", option_container, option_view);

    // Example 4: Result container
    let result_container: Result<i32, String> = Ok(200);
    let result_view: Result<&i32, &String> = result_container.view();
    println!("Example 4 (Result): container = {:?}, view = {:?}", result_container, result_view);

    // Example 5: Array container
    let array_container: [i32; 3] = [10, 20, 30];
    let array_view: &[i32] = array_container.view();
    println!("Example 5 (Array): container = {:?}, view = {:?}", array_container, array_view);

    println!("\nAll 5 GAT Container examples compiled successfully!");
    println!("\nKey GAT benefits demonstrated:");
    println!("  1. Generic containers - works with Vec, Box, Option, Result, Array");
    println!("  2. Borrowed views - each returns a reference with proper lifetime tracking");
    println!("  3. Zero-cost abstractions - monomorphized at compile time");
}

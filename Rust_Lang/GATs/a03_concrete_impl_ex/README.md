# Result

```bash
=== Example 1: Basic integer iteration ===
Got: 10
Got: 20
Got: 30
Got: 40
Got: 50

=== Example 2: String iteration ===
Word: hello
Word: world
Word: GATs
Word: Rust

=== Example 3: Multiple iterators ===
First iterator: Some(1)
Second iterator: Some(1)
First iterator again: Some(2)

=== Example 4: Custom type iteration ===
Point: Point { x: 0, y: 0 }
Point: Point { x: 1, y: 2 }
Point: Point { x: 3, y: 4 }

=== Example 5: Partial iteration ===
First 2 items:
  Some(100)
  Some(200)
Resume after break:
  300
  400
  500

```


# ⏺ All 5 examples have been added to the main function:

- 1. Example 1: Basic integer iteration - Simple iteration over a vector of integers
- 2. Example 2: String iteration - Iterating over string slices
- 3. Example 3: Multiple iterators - Shows zero-copy borrowing with two independent iterators from the same data
- 4. Example 4: Custom type iteration - Using a custom Point struct with Debug trait
- 5. Example 5: Partial iteration - Demonstrating partial iteration and resuming

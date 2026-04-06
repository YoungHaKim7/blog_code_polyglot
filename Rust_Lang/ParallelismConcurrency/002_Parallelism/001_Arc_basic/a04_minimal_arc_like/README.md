# Result


```bash
=== Example 1: Basic Usage ===
Reference count: 3
Data: Hello, MyArc!

Example 1 completed - all clones dropped

=== Example 2: Multi-threaded Usage ===
Thread 0: [1, 2, 3, 4, 5], count: 4
Thread 1: [1, 2, 3, 4, 5], count: 4
Thread 2: [1, 2, 3, 4, 5], count: 4
Main: Final count: 1

=== Example 3: Custom Type ===
Initial counter: Counter { value: 0 }
With 4 total references, count: 4
MyArc { ptr: 0x104de6340 } MyArc { ptr: 0x104de6340 } MyArc { ptr: 0x104de6340 }
```


# fix

- Done! I've added 3 examples to `main.rs`:

1. **Example 1 - Basic Usage**: Shows creating a `MyArc` with a String, cloning it twice, and demonstrating that all three point to the same data with a reference count of 3.

2. **Example 2 - Multi-threaded Usage**: Demonstrates the thread-safe nature of `MyArc` by spawning 3 threads that share the same data, showing the reference count changing as threads are spawned and complete.

3. **Example 3 - Custom Type**: Shows `MyArc` working with a custom `Counter` struct, demonstrating that it works with any type.

I also added helper methods `get_data()` and `ref_count()` to make the examples cleaner and avoid unsafe code in the main function.

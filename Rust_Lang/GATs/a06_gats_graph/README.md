# Result

```bash
╔════════════════════════════════════════════════════════════╗
║   GATs in Graph / Iterator APIs - Full Examples           ║
╚════════════════════════════════════════════════════════════╝

Example 1: GAT-based Graph trait
────────────────────────────────────────
✓ GAT allows lifetime to be tied to &self
✓ type Neighbors<'a>: Iterator<Item = Self::Node>
  where Self: 'a;

Example 2: Pre-GAT approach (avoided)
────────────────────────────────────────
✗ Had to return: Box<dyn Iterator<Item = Self::Node> + 'a>
✗ Heap allocation required
✗ Dynamic dispatch (vtable lookup)

Example 3: StreamingIterator pattern
────────────────────────────────────────
✓ trait StreamingIterator {
    type Item<'a> where Self: 'a;
    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
  }
✓ Enables borrowing from iterator itself

Example 4: Concrete graph usage
────────────────────────────────────────
  Node 0 neighbors: 1 2
  Node 1 neighbors: 2
  Node 2 neighbors: 0 1

Example 5: Size comparison (Zero-cost abstraction)
─────────────────────────────────────────────────
  GAT NeighborsIter<'a>:     24 bytes
  Box<dyn Iterator>:         16 bytes

Benefits Summary:
  ✓ No heap allocation (stack-allocated iterator)
  ✓ No Box<dyn Iterator> (concrete type)
  ✓ Fully static dispatch (monomorphization)
  ✓ Zero-cost abstraction

```


# ⏺ The code compiles cleanly with no warnings. Here's what was implemented:

- All 5 Examples in main.rs:
  - 1. GAT-based Graph trait - Shows the modern way to define graph APIs with lifetime-parameterized associated types
  - 2. Pre-GAT approach - Demonstrates what we avoid (Box with heap allocation and dynamic dispatch)
  - 3. StreamingIterator pattern - Shows an alternative GAT design pattern for borrowing from iterators
  - 4. Concrete implementation - SimpleGraph with NeighborsIter that returns a stack-allocated iterator (no heap allocation)
  - 5. Size comparison - Shows the zero-cost abstraction benefit

- Key Benefits Demonstrated:
  - No heap allocation (stack-allocated `NeighborsIter<'a>`)
  - No `Box<dyn Iterator>` (concrete type returned)
  - Fully static dispatch (monomorphization)
  - Zero-cost abstraction

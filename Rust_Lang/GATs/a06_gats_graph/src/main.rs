// ==================== GATs in Graph / Iterator APIs ====================
// Benefits: No heap allocation, No Box<dyn Iterator>, Fully static dispatch


// ============================================================================
// Example 1: GAT-based Graph trait (The Modern Way)
// ============================================================================
trait Graph {
    type Node;
    // GAT allows lifetime to be tied to &self
    type Neighbors<'a>: Iterator<Item = Self::Node>
    where
        Self: 'a;

    fn neighbors<'a>(&'a self, n: Self::Node) -> Self::Neighbors<'a>;
}

// ============================================================================
// Example 2: Pre-GAT approach (What we avoid - Box<dyn Iterator>)
// ============================================================================
#[allow(dead_code)]
trait GraphOld {
    type Node;

    // Had to return boxed trait object - heap allocation, dynamic dispatch
    fn neighbors<'a>(&'a self, n: Self::Node) -> Box<dyn Iterator<Item = Self::Node> + 'a>;
}

// ============================================================================
// Example 3: GAT-based StreamingIterator (Alternative design pattern)
// ============================================================================
#[allow(dead_code)]
trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// ============================================================================
// Example 4: Concrete Graph Implementation using GATs
// ============================================================================
struct SimpleGraph {
    edges: Vec<(usize, Vec<usize>)>,
}

impl SimpleGraph {
    fn new() -> Self {
        SimpleGraph {
            edges: vec![(0, vec![1, 2]), (1, vec![2]), (2, vec![0, 1])],
        }
    }
}

// Our custom iterator type - zero-cost, stack-allocated
struct NeighborsIter<'a> {
    graph: &'a SimpleGraph,
    node: usize,
    index: usize,
}

impl<'a> Iterator for NeighborsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        for (n, neighbors) in &self.graph.edges {
            if *n == self.node {
                if self.index < neighbors.len() {
                    let item = neighbors[self.index];
                    self.index += 1;
                    return Some(item);
                } else {
                    return None;
                }
            }
        }
        None
    }
}

impl Graph for SimpleGraph {
    type Node = usize;
    // No heap allocation! Returns a concrete stack-allocated iterator
    type Neighbors<'a> = NeighborsIter<'a> where Self: 'a;

    fn neighbors<'a>(&'a self, n: Self::Node) -> Self::Neighbors<'a> {
        NeighborsIter {
            graph: self,
            node: n,
            index: 0,
        }
    }
}

// ============================================================================
// Example 5: Size comparison showing zero-cost abstraction
// ============================================================================

// This is what we'd have to do WITHOUT GATs - Box<dyn Iterator>
// Note: This type is larger due to heap pointer and vtable

// ============================================================================
// Main: Running all 5 examples
// ============================================================================
fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║   GATs in Graph / Iterator APIs - Full Examples           ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Example 1: Show the GAT trait definition works
    println!("Example 1: GAT-based Graph trait");
    println!("────────────────────────────────────────");
    println!("✓ GAT allows lifetime to be tied to &self");
    println!("✓ type Neighbors<'a>: Iterator<Item = Self::Node>");
    println!("  where Self: 'a;\n");

    // Example 2: Compare with old Box approach
    println!("Example 2: Pre-GAT approach (avoided)");
    println!("────────────────────────────────────────");
    println!("✗ Had to return: Box<dyn Iterator<Item = Self::Node> + 'a>");
    println!("✗ Heap allocation required");
    println!("✗ Dynamic dispatch (vtable lookup)\n");

    // Example 3: StreamingIterator pattern
    println!("Example 3: StreamingIterator pattern");
    println!("────────────────────────────────────────");
    println!("✓ trait StreamingIterator {{");
    println!("    type Item<'a> where Self: 'a;");
    println!("    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;");
    println!("  }}");
    println!("✓ Enables borrowing from iterator itself\n");

    // Example 4: Concrete usage
    println!("Example 4: Concrete graph usage");
    println!("────────────────────────────────────────");
    let graph = SimpleGraph::new();

    for node in 0..=2 {
        print!("  Node {} neighbors: ", node);
        for neighbor in graph.neighbors(node) {
            print!("{} ", neighbor);
        }
        println!();
    }
    println!();

    // Example 5: Size comparison
    println!("Example 5: Size comparison (Zero-cost abstraction)");
    println!("─────────────────────────────────────────────────");

    // Show sizes
    println!("  GAT NeighborsIter<'a>:     {} bytes", std::mem::size_of::<NeighborsIter>());
    println!("  Box<dyn Iterator>:         {} bytes", std::mem::size_of::<Box<dyn Iterator<Item = usize>>>());
    println!();

    println!("Benefits Summary:");
    println!("  ✓ No heap allocation (stack-allocated iterator)");
    println!("  ✓ No Box<dyn Iterator> (concrete type)");
    println!("  ✓ Fully static dispatch (monomorphization)");
    println!("  ✓ Zero-cost abstraction");
}

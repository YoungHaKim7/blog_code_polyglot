// =====================================================
// CONTRAVARIANCE DEMONSTRATION in Rust
// =====================================================
//
// Function parameter types are CONTRAVARIANT
// This means: if T <: U, then fn(U) <: fn(T) (relationship flips!)
//
// For lifetimes: 'short <: 'long <: 'static (covariant - shorter is subtype of longer)
// Therefore:   fn(&'static) <: fn(&'long) <: fn(&'short) (contravariant - flipped!)

fn main() {
    // ================================================================
    // KEY INSIGHT: Understanding Contravariance Direction
    // ================================================================
    //
    // Which function type is "more permissive"?
    //
    // fn(&'long i32)  vs  fn(&'short i32)
    //
    // Answer: fn(&'long i32) is MORE PERMISSIVE!
    // Why? Because:
    //   - fn(&'long) CAN be called with &'short  (shorter coerces to longer) ✓
    //   - fn(&'short) CANNOT be called with &'long (longer won't coerce to shorter) ✗
    //
    // So: fn(&'long) <: fn(&'short)  (relationship is FLIPPED!)

    // ================================================================
    // DEMONSTRATION: Contravariance error with function pointers
    // ================================================================

    fn takes_static(x: &'static i32) -> i32 {
        *x
    }

    fn takes_any<'a>(x: &'a i32) -> i32 {
        *x
    }

    // Function pointer types
    type FnStatic = fn(&'static i32) -> i32;

    // takes_static has type: fn(&'static i32) -> i32
    // takes_any has type:    for<'a> fn(&'a i32) -> i32

    // Assignment 1: Works because takes_any can handle 'static
    let _f1: FnStatic = takes_any; // OK ✓

    // Assignment 2: ERROR! takes_static is more restrictive
    // let _f2: fn(&i32) = takes_static;  // ERROR ✗
    //
    // Error message:
    // "expected fn pointer `for<'a> fn(&'a _) -> _`,
    //  found fn item `fn(&'static _) -> _`"
    //
    // Why? Because:
    // - The target type accepts ANY lifetime ('a)
    // - takes_static only accepts 'static
    // - Cannot use a MORE restrictive function where a MORE permissive one is expected
    //
    // This is contravariance in action!

    // ================================================================
    // DEMONSTRATION 2: Visualizing the relationship
    // ================================================================

    // Lifetime hierarchy (covariant):
    // 'short <: 'long <: 'static
    // (shorter lifetimes are subtypes of longer ones)

    // Function pointer hierarchy (contravariant - FLIPPED!):
    // fn(&'static) <: fn(&'long) <: fn(&'short)
    // (functions accepting longer lifetimes are subtypes of functions accepting shorter ones)

    // Why the flip?
    //
    // fn(&'long) is MORE PERMISSIVE because it can accept:
    //   - &'long
    //   - &'short (coerces to &'long)
    //
    // fn(&'short) is LESS PERMISSIVE because it can only accept:
    //   - &'short
    //   - NOT &'long (won't coerce)

    // Therefore: fn(&'long) <: fn(&'short) (subtype relationship!)

    // ================================================================
    // DEMONSTRATION 3: Type coercion examples
    // ================================================================

    // Example showing lifetime coercion
    fn demonstrate_coercion() {
        let x: i32 = 42;
        let short_ref: &i32 = &x;

        // Functions with longer lifetime parameters can accept shorter references
        // This is why fn(&'long) is more permissive than fn(&'short)
        let callback: fn(&i32) -> i32 = |val: &i32| *val;
        let _ = callback(&x); // Works - any reference can be passed
        let _ = callback(short_ref); // Also works
    }

    // ================================================================
    // Summary: Variance Rules in Rust
    // ================================================================
    //
    // Type Constructor   | Variance in Lifetime | Example
    // -------------------|----------------------|------------------------
    // &T                 | Covariant            | &'long <: &'short
    // &mut T             | Covariant (lifetime) | &'long mut <: &'short mut
    // fn(&T)             | CONTRAVARIANT        | fn(&'short) <: fn(&'long)
    // fn() -> &T         | Covariant            | fn()->&'long <: fn()->&'short
    // Box<T>             | Covariant            | Box<Cat> <: Box<Animal>
    // Cell<T>            | Invariant            | No subtyping
    // UnsafeCell<T>      | Invariant            | No subtyping
    //
    // Key mnemonic:
    // - INPUT positions  = Contravariant (flips relationship)
    // - OUTPUT positions = Covariant (preserves relationship)
    // - BOTH input+output = Invariant (no relationship)

    println!("Contravariance demonstration complete!");
    println!();
    println!("Key insight:");
    println!("  'short <: 'long     (covariant)");
    println!("  fn(&'long) <: fn(&'short)  (CONTRAVARIANT - flipped!)");
}

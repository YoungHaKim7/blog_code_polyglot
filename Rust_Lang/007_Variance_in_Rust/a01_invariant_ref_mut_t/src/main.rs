// ============================================================
// Example 1: Basic invariant behavior with &mut T
// ============================================================

#[allow(dead_code)]
fn takes_long<'long>(x: &mut &'long i32) -> i32 {
    let add = 20;
    *x + &add
}

fn takes_long02<'long>(x: &'long i32) -> i32 {
    let add = 20;
    x + add
}

// This would be ALLOWED if &mut T were covariant (but it's NOT!)
#[allow(dead_code)]
fn demonstrate_invariant_basic() {
    let value = 50;
    let mut r: &i32 = &value;

    // ❌ ERROR: lifetime mismatch
    // 'short != 'long - cannot coerce &i32 to &'long i32 through &mut
    let res = takes_long(&mut r);

    println!("{res:?}");
}

// ============================================================
// Example 2: Why invariance is NECESSARY for safety
// ============================================================

#[allow(dead_code)]
struct Container<'a> {
    data: &'a i32,
}

// If &mut T were covariant, this would be ALLOWED (UNSAFE!)
#[allow(dead_code)]
fn unsafe_scenario() {
    let outer_ref: &i32;
    let short_lived = 10;

    {
        let container = Container { data: &short_lived };
        // If this worked (covariance), we'd have a problem:
        outer_ref = container.data; // 'short -> 'long coercion
    } // short_lived dropped here

    // outer_ref now points to DROPPED data! 💀 Use-after-free
    println!("{}", *outer_ref);
}

// ============================================================
// Example 3: Demonstrating the write safety problem
// ============================================================

#[allow(dead_code)]
fn covariant_would_allow_write_unsafe() {
    // If &mut<'a, T> were covariant in T:
    // &mut<'static, Dog> could coerce to &mut<'short, Animal>

    trait Animal {
        fn make_sound(&self);
    }

    struct Dog;
    impl Animal for Dog {
        fn make_sound(&self) {
            println!("Woof!");
        }
    }

    struct Cat;
    impl Animal for Cat {
        fn make_sound(&self) {
            println!("Meow!");
        }
    }

    // Hypothetical: if covariant, &mut Dog -> &mut dyn Animal
    // let animal_ref: &mut dyn Animal = &mut Dog;

    // Now we could write a Cat into a Dog variable! 💀
    // *animal_ref = Cat;

    // dog.make_sound(); // Would print "Meow!" - type confusion!
}

// ============================================================
// Example 4: Invariance means exact match required
// ============================================================

#[allow(dead_code)]
fn exact_lifetime_match_required<'a>(r: &'a i32) -> &'a i32 {
    r
}

// This shows why invariance exists - prevents writing shorter-lived data
#[allow(dead_code)]
#[allow(unused_variables)]
fn demonstrate_exact_match<'a, 'b>(x: &'a mut &'b i32, long_lived: &'b i32) {
    let local = 42;

    // ❌ ERROR: lifetime mismatch
    // &mut T is invariant - we CANNOT write shorter lifetime into it
    // *x = &local;  // This would create dangling pointer!

    // ✅ This works - exact lifetime match
    *x = long_lived;
}

// ============================================================
// Example 5: Practical example with cells
// ============================================================

use std::cell::RefCell;

struct Context<'a> {
    shared: RefCell<&'a i32>,
}

fn demonstrate_cell_invariance() {
    let data = 100;
    let ctx = Context {
        shared: RefCell::new(&data),
    };

    // ❌ This would be UNSAFE if allowed:
    // let short = 50;
    // ctx.shared = RefCell::new(&short);
    // Because ctx expects &'long i32, but we'd give &'short i32

    println!("{:?}", ctx.shared.borrow());
}

// ============================================================
// Example 6: Contrast - &T is covariant (works!)
// ============================================================

fn takes_shared<'a, 'long>(x: &'a &'long i32) -> i32 {
    **x + 100
}

fn demonstrate_shared_covariance() {
    let value = 50;
    let r: &i32 = &value;

    // ✅ WORKS! &T is covariant
    // &i32 can coerce to &'long i32 through shared reference
    let res = takes_shared(&r);

    println!("{res}");
}

// ============================================================
// Main - comment/uncomment examples to test
// ============================================================

#[allow(unused_variables)]
fn main() {
    // demonstrate_invariant_basic();  // ❌ Uncomment to see error
    println!("{}", takes_long02(&100));
    let _long = 1000;
    // demonstrate_exact_match(&mut (&_long), &_long);

    demonstrate_cell_invariance();

    demonstrate_shared_covariance();
}

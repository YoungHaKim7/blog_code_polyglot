use std::cell::Cell;

// ============================================================
// VARIANCE ANNOTATIONS - Why each field is covariant/invariant/contravariant
// ============================================================

#[derive(Debug)]
struct MyType<'a, 'b, A: 'a, B: 'b, C, D, E, F, G: Copy, H: Copy, In, Out, Mixed> {
    // ────────────────────────────────────────────────────────────────
    // a: &'a A  →  Covariant over 'a AND Covariant over A
    // ────────────────────────────────────────────────────────────────
    // WHY: Immutable reference can be treated as a shorter-lived reference
    // and can read from more specific types (subtyping works both ways)
    //
    // Example: &'static str can be used where &'a str is expected
    //          (longer lifetime → shorter lifetime is OK)
    //
    // ✅ WORKS: &'static i32 → &'a i32  (lifetime covariance)
    // ✅ WORKS: &'a Animal → &'a Dog   (type covariance - read only)
    a: &'a A,

    // ────────────────────────────────────────────────────────────────
    // b: &'b mut B  →  Covariant over 'b, INVARIANT over B
    // ────────────────────────────────────────────────────────────────
    // WHY: Can shorten lifetime, but CANNOT change type B
    //
    // ❌ NOT OK to be covariant over B because:
    //    If &'b mut Dog could become &'b mut Animal,
    //    you could write a Cat into the Dog's memory!
    //
    // Example of unsoundness if covariant:
    //   let mut dog: Dog = ...;
    //   let ref_mut: &mut Dog = &mut dog;
    //   let ref_animal: &mut Animal = ref_mut;  // ❌ If allowed...
    //   *ref_animal = Cat;  // 😱 Now dog is a Cat!
    b: &'b mut B,

    // ────────────────────────────────────────────────────────────────
    // c: *const C  →  Covariant over C
    // ────────────────────────────────────────────────────────────────
    // WHY: Read-only pointer, like &T - can read from more specific types
    // ✅ Similar to&T: safe to treat *const Dog as *const Animal
    c: *const C,

    // ────────────────────────────────────────────────────────────────
    // d: *mut D  →  INVARIANT over D
    // ────────────────────────────────────────────────────────────────
    // WHY: Mutable pointer allows writing, same issue as &mut T
    //
    // ❌ If *mut Dog could become *mut Animal:
    //    let mut dog: Dog = ...;
    //    let ptr: *mut Dog = &mut dog;
    //    let ptr_animal: *mut Animal = ptr;  // ❌ If allowed...
    //    *ptr_animal = Cat;  // 😱 Type confusion!
    d: *mut D,

    // ────────────────────────────────────────────────────────────────
    // e: E  →  Covariant over E
    // ────────────────────────────────────────────────────────────────
    // WHY: Owned value, can consume Dog where Animal is expected
    // ✅ MyType<Dog> can be used where MyType<Animal> is needed
    e: E,

    // ────────────────────────────────────────────────────────────────
    // f: Vec<F>  →  Covariant over F
    // ────────────────────────────────────────────────────────────────
    // WHY: Vec is like owned collection - only way to access T is by
    //      moving it out (which consumes Vec) or getting &T references
    // ✅ Vec<Dog> → Vec<Animal> is safe
    f: Vec<F>,

    // ────────────────────────────────────────────────────────────────
    // g: Cell<G>  →  INVARIANT over G
    // ────────────────────────────────────────────────────────────────
    // WHY: Cell allows interior mutation via &Cell<T>
    //
    // ❌ If Cell<Dog> could become Cell<Animal>:
    //    let dog_cell: Cell<Dog> = Cell::new(Dog);
    //    let animal_cell: &Cell<Animal> = &dog_cell;  // ❌ If allowed...
    //    animal_cell.set(Cat);  // 😱 dog_cell now contains Cat!
    //
    // KEY RULE: Any type with interior mutability (&T can modify T) is INVARIANT
    g: Cell<G>,

    // ────────────────────────────────────────────────────────────────
    // h1: H  →  Covariant over H (normally)
    // h2: Cell<H>  →  Invariant over H
    // RESULT:  INVARIANT over H (invariance WINS all conflicts)
    // ────────────────────────────────────────────────────────────────
    // WHY: When a type parameter appears in both covariant AND invariant
    //      positions, the overall variance is INVARIANT
    //
    // Rule: Covariant × Invariant = Invariant
    //
    // This prevents the unsound conversion through the invariant path
    h1: H,
    h2: Cell<H>,

    // ────────────────────────────────────────────────────────────────
    // i: fn(In) -> Out  →  CONTRAVARIANT over In, Covariant over Out
    // ────────────────────────────────────────────────────────────────
    // WHY: Function follows Liskov substitution principle backwards
    //
    // Input (In) is CONTRAVARIANT:
    //   fn(Animal) -> T  can be used where fn(Dog) -> T  is expected
    //   (accepting more general type is OK when more specific is needed)
    //
    // Output (Out) is COVARIANT:
    //   fn() -> Dog  can be used where fn() -> Animal  is expected
    //   (returning more specific type is OK)
    //
    // Remember: "Consumer is contravariant, Producer is covariant"
    i: fn(In) -> Out,

    // ────────────────────────────────────────────────────────────────
    // k1: fn(Mixed) -> usize  →  Contravariant over Mixed (normally)
    // k2: Mixed  →  Covariant over Mixed (normally)
    // RESULT:  INVARIANT over Mixed (invariance WINS all conflicts)
    // ────────────────────────────────────────────────────────────────
    // Same rule: when type appears in multiple positions with different
    // variances, the result is always INVARIANT
    //
    // Rule: Contravariant × Covariant = Invariant
    k1: fn(Mixed) -> usize,
    k2: Mixed,
}

fn main() {
    // ============================================================
    // ORIGINAL CODE THAT DIDN'T WORK:
    // Variables a, b, c, d, e, f, g, h1, h2, i, k1, k2
    // were never defined!
    // ============================================================

    // Working example with concrete types:
    let x: i32 = 42;
    let a: &i32 = &x;

    let mut y: i32 = 10;
    let b: &mut i32 = &mut y;

    let c: *const i32 = &x as *const i32;
    // Note: Using separate variable to avoid multiple mutable borrows
    let mut z: i32 = 20;
    let d: *mut i32 = &mut z as *mut i32;

    let e: i32 = 100;
    let f: Vec<i32> = vec![1, 2, 3];
    let g: Cell<i32> = Cell::new(5);

    let h1: i32 = 50;
    let h2: Cell<i32> = Cell::new(25);

    let i: fn(i32) -> i32 = |x| x * 2;

    let k1: fn(i32) -> usize = |x| x as usize;
    let k2: i32 = 99;

    let my_data = MyType {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h1,
        h2,
        i,
        k1,
        k2,
    };

    println!("{my_data:?}");
}

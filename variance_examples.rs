// Variance in Rust — Companion code for the variance guide.
//
// Compile & run:  rustc variance_examples.rs && ./variance_examples
// Minimum: Rust 2021 edition (stable). No nightly features required.
//
// KEY DIFFERENCE FROM JAVA/KOTLIN:
// In Java/Kotlin, variance is about type inheritance (Cat extends Animal).
// In Rust, variance is about *lifetimes* ('long: 'short) and is enforced
// by the borrow checker at compile time — no runtime checks, no garbage collector.
//
// Subtyping in Rust:
//   Java/Kotlin:  Cat extends Animal     → Cat is a subtype of Animal
//   Rust:         'long: 'short          → 'long is a subtype of 'short
//   (A longer lifetime is "more specific" — it makes a stronger guarantee.)

use std::fmt::Display;
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// 1. Covariance: Lifetimes
//    Rust's &'a T is covariant over 'a.
//    Equivalent to: Kotlin's `out` / Java's `? extends`
// ---------------------------------------------------------------------------

fn covariance_lifetimes() {
    // A 'static string literal — the longest possible lifetime.
    let long_lived: &'static str = "I live forever";

    // We can assign it to a reference with a *shorter* lifetime.
    // This is covariance: &'long str → &'short str is safe because
    // the data will still be alive when the shorter reference expires.
    let short_lived: &str = long_lived; // ✅ Covariance!

    assert_eq!(short_lived, "I live forever");
    println!("  &'static str assigned to &'short str: \"{}\"", short_lived);

    // Why is this safe? A reference that lives 10 seconds can always be used
    // where a 5-second reference is expected — the data outlives the borrow.
    // This is exactly like Kotlin's Cage<out Cat> being assignable to Cage<out Animal>.
}

// Helper: prints each string reference in a slice.
fn print_labels(labels: &[&str]) {
    for (i, label) in labels.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{}", label);
    }
    println!();
}

// ---------------------------------------------------------------------------
// 2. Covariance: Containers
//    Box<T>, Vec<T>, Option<T> are all covariant over T.
//    Like Kotlin's List<out E> — read-only containers that safely "narrow."
// ---------------------------------------------------------------------------

fn covariance_containers() {
    // Vec<&'static str> can be passed where &[&'a str] is expected,
    // because &'static str is a subtype of &'a str for any 'a.
    let static_vec: Vec<&'static str> = vec!["hello", "world"];
    print!("  Vec<&'static str> as &[&str]: ");
    print_labels(&static_vec); // ✅ Vec<&'static str> coerces to &[&'short str]

    // Box is also covariant over T:
    let boxed: Box<&'static str> = Box::new("boxed forever");
    let unboxed: &str = *boxed; // ✅ Covariance lets us use the 'static value
    assert_eq!(unboxed, "boxed forever");
    println!("  Box<&'static str> unboxed: \"{}\"", unboxed);

    // Option<T> is covariant too:
    let opt: Option<&'static str> = Some("optional");
    let short_opt: Option<&str> = opt; // ✅ Covariance
    assert_eq!(short_opt, Some("optional"));
    println!("  Option<&'static str> coerced: {:?}", short_opt);
}

// ---------------------------------------------------------------------------
// 3. Contravariance: Function Parameters
//    fn(T) is contravariant over T.
//    Equivalent to: Kotlin's `in` / Java's `? super`
//    The direction REVERSES — a function accepting broader input can
//    substitute for one expecting narrower input.
// ---------------------------------------------------------------------------

fn handle_any(s: &str) -> usize {
    // This function accepts &str with ANY lifetime.
    s.len()
}

fn needs_static_handler(f: fn(&'static str) -> usize) -> usize {
    // This function requires a handler that works on 'static strings.
    f("this string lives forever")
}

fn contravariance_functions() {
    // handle_any accepts &'a str for ANY 'a — including 'static.
    // So it's safe to use where fn(&'static str) is expected.
    // This is CONTRAVARIANCE: the input type relationship REVERSES.
    //
    // Think of it like Kotlin's Handler<in Animal> being assignable to Handler<in Cat>:
    // A handler for ALL animals can certainly handle cats.
    // A function for ALL lifetimes can certainly handle 'static.
    let result = needs_static_handler(handle_any); // ✅ Contravariance!
    assert_eq!(result, "this string lives forever".len());
    println!("  fn(&str) used as fn(&'static str): length = {}", result);
}

// ---------------------------------------------------------------------------
// 4. Invariance: &mut T (THE KEY TO RUST'S MEMORY SAFETY)
//    &mut T is invariant over T.
//    Equivalent to: Kotlin's MutableList<T> (no modifier) / Java's plain T
//    When you can both read AND write, no substitution is safe.
// ---------------------------------------------------------------------------

fn invariance_mut_refs() {
    // Working example: &mut works when types match exactly.
    let mut greeting: &str = "hello";
    {
        let r: &mut &str = &mut greeting;
        *r = "world"; // ✅ Same type, same lifetime — fine.
    }
    assert_eq!(greeting, "world");
    println!("  &mut &str mutation (same type): \"{}\"", greeting);

    // WHY is &mut T invariant? Here's what would go wrong if it weren't:
    //
    // DANGEROUS SCENARIO (does NOT compile — Rust prevents this):
    //
    //   let mut forever: &'static str = "I live forever";
    //   {
    //       let mut ref_to_forever: &mut &str = &mut forever;
    //
    //       // Step 1: ref_to_forever has type &mut &'static str
    //       // Step 2: IF &mut T were COVARIANT, we could treat it as &mut &'short str
    //       // Step 3: Then we could write a short-lived reference through it:
    //
    //       let temp = String::from("I die soon");
    //       *ref_to_forever = &temp;  // Would write a short-lived ref!
    //
    //       // Step 4: `forever` still claims to be &'static str...
    //   }
    //   // Step 5: `temp` is dropped here.
    //   // `forever` now points to freed memory.
    //   // println!("{}", forever);  // 💥 USE-AFTER-FREE!
    //
    // The compiler prevents this because &mut T is INVARIANT over T.
    // This is the exact same principle as Kotlin's MutableList<Cat> NOT being
    // assignable to MutableList<Animal> — mutation makes covariance unsafe.

    println!("  &mut T is invariant: compiler prevents lifetime smuggling");
    println!("  (Also invariant: Cell<T>, RefCell<T>, UnsafeCell<T>, *mut T)");
}

// ---------------------------------------------------------------------------
// 5. PhantomData: Manual Variance Control
//    When a type parameter doesn't appear in any field, the compiler can't
//    determine variance. PhantomData tells it what you intend.
// ---------------------------------------------------------------------------

/// Covariant over V — like Kotlin's `out V`.
/// PhantomData<V> pretends the struct holds a V, making it covariant.
#[allow(dead_code)]
struct CovariantKey<V> {
    raw: String,
    _type: PhantomData<V>, // covariant: "I produce V"
}

/// Contravariant over V — like Kotlin's `in V`.
/// PhantomData<fn(V)> pretends the struct consumes V, making it contravariant.
#[allow(dead_code)]
struct ContravariantKey<V> {
    raw: String,
    _type: PhantomData<fn(V)>, // contravariant: "I consume V"
}

/// Invariant over V — like Kotlin's no modifier (read + write).
/// PhantomData<fn(V) -> V> pretends the struct both consumes and produces V.
#[allow(dead_code)]
struct InvariantKey<V> {
    raw: String,
    _type: PhantomData<fn(V) -> V>, // invariant: "I do both"
}

fn phantom_data_variance() {
    // Practical usage: a typed key that carries type information at zero cost.
    let key: CovariantKey<i32> = CovariantKey {
        raw: "user:score".to_string(),
        _type: PhantomData,
    };
    assert_eq!(key.raw, "user:score");
    println!("  CovariantKey<i32>:      PhantomData<V>          (like Kotlin out)");

    let _ckey: ContravariantKey<i32> = ContravariantKey {
        raw: "handler:int".to_string(),
        _type: PhantomData,
    };
    println!("  ContravariantKey<i32>:  PhantomData<fn(V)>      (like Kotlin in)");

    let _ikey: InvariantKey<i32> = InvariantKey {
        raw: "cell:int".to_string(),
        _type: PhantomData,
    };
    println!("  InvariantKey<i32>:     PhantomData<fn(V) -> V>  (like Kotlin no modifier)");

    // PhantomData cheat sheet:
    //   PhantomData<T>            → covariant     (like holding a T)
    //   PhantomData<fn(T)>        → contravariant (like consuming a T)
    //   PhantomData<fn(T) -> T>   → invariant     (like doing both)
}

// ---------------------------------------------------------------------------
// 6. Covariance: Slices and Arrays
//    [T] and [T; n] are covariant over T — just like Vec<T>.
//    (From the official Rust Reference variance table.)
// ---------------------------------------------------------------------------

fn covariance_slices_and_arrays() {
    // Slices [T] are covariant over T:
    let static_strs: [&'static str; 2] = ["hello", "world"];
    let short_slice: &[&str] = &static_strs; // ✅ [&'static str; 2] → &[&'short str]
    assert_eq!(short_slice.len(), 2);
    println!("  [&'static str; 2] as &[&str]: {:?}", short_slice);

    // Arrays [T; n] are also covariant:
    let arr: [&'static str; 3] = ["a", "b", "c"];
    let coerced: &[&str] = &arr; // ✅ Covariance
    assert_eq!(coerced[0], "a");
    println!("  Arrays and slices are covariant, just like Vec<T>");
}

// ---------------------------------------------------------------------------
// 7. Struct Variance Composition
//    A struct's variance over a parameter is determined by how ALL its fields
//    use that parameter. Invariance wins all conflicts.
//    (From the Rustonomicon.)
// ---------------------------------------------------------------------------

use std::cell::Cell;

/// This struct demonstrates how variance is computed from fields.
/// 'a appears in two fields with DIFFERENT variance:
///   - &'a T makes it covariant over 'a
///   - Cell<&'a i32> makes it invariant over 'a
/// Result: invariant over 'a (invariance wins all conflicts).
#[allow(dead_code)]
struct MixedVariance<'a> {
    readable: &'a str,         // covariant over 'a
    mutable_cell: Cell<&'a i32>, // invariant over 'a (Cell allows interior mutation)
    // Combined: INVARIANT over 'a — invariance always wins.
}

fn struct_variance_composition() {
    // THE RULE: A struct inherits the variance of its fields.
    //   - All covariant uses     → covariant
    //   - All contravariant uses → contravariant
    //   - Mixed uses             → INVARIANT (invariance wins all conflicts)
    //
    // This is how the compiler determines variance for YOUR custom structs.
    // You never declare variance explicitly — it's derived from field types.

    println!("  Struct variance rules:");
    println!("    All fields covariant     → struct is covariant");
    println!("    All fields contravariant → struct is contravariant");
    println!("    Mixed or any invariant   → struct is INVARIANT");
    println!("  Example: struct with &'a str (co) + Cell<&'a i32> (inv) → invariant over 'a");
}

// ---------------------------------------------------------------------------
// 8. Trait Objects: Lifetime and Type Variance
//    Box<dyn Trait + 'a> is covariant over 'a but INVARIANT over T.
//    (From the official Rust Reference variance table.)
// ---------------------------------------------------------------------------

fn print_display(item: Box<dyn Display + '_>) {
    println!("  Trait object says: {}", item);
}

fn trait_object_variance() {
    // COVARIANT over 'a: Box<dyn Display + 'static> can be used where
    // Box<dyn Display + 'a> is expected, because 'static outlives any 'a.
    let boxed: Box<dyn Display + 'static> = Box::new("I am a static trait object");
    print_display(boxed); // ✅ 'static coerces to shorter lifetime

    // Also works with non-'static data:
    let local = String::from("I am a local trait object");
    let boxed_local: Box<dyn Display + '_> = Box::new(local);
    print_display(boxed_local); // ✅ Lifetime matches naturally

    println!("  Covariant over 'a: Box<dyn Trait + 'static> → Box<dyn Trait + '_>");

    // INVARIANT over T: dyn Trait<T> is invariant over its type parameter.
    // This means Box<dyn Trait<Cat>> is NOT a subtype of Box<dyn Trait<Animal>>,
    // even if Cat were a subtype of Animal.
    // Why? Trait objects erase the concrete type, and the vtable is fixed —
    // the compiler can't safely assume substitutability for type parameters.
    println!("  Invariant over T: dyn Trait<T> does NOT allow type substitution");
}

// ---------------------------------------------------------------------------
// 9. Common Lifetime Error: The Same-Lifetime Trap
//    This error is CAUSED BY VARIANCE — specifically &mut T's invariance.
// ---------------------------------------------------------------------------

// ❌ BAD: Same lifetime for the mutable borrow AND the contents.
//
// struct CacheBad<'a> {
//     data: &'a mut Vec<&'a str>,
// }
//
// WHY THIS FAILS:
// - &'a mut makes Vec INVARIANT over 'a (because &mut T is invariant over T)
// - But Vec<&'a str> needs 'a to be COVARIANT (shorter refs should be accepted)
// - Same 'a with conflicting variance requirements → borrow checker error!
// - You'll get confusing errors like "borrowed value does not live long enough"

// ✅ GOOD: Separate lifetimes let each follow its own variance rule.
struct Cache<'a, 'b> {
    data: &'a mut Vec<&'b str>,
    // 'a = lifetime of the mutable borrow (invariant — locked)
    // 'b = lifetime of the string references (covariant — can flex)
    // No conflict!
}

fn common_lifetime_error_and_fix() {
    let mut storage: Vec<&str> = vec!["cached"];

    {
        let cache = Cache { data: &mut storage };
        cache.data.push("new entry"); // ✅ Works with split lifetimes
        assert_eq!(cache.data.len(), 2);
    }

    // storage is usable again after the mutable borrow ends.
    assert_eq!(storage, vec!["cached", "new entry"]);
    println!("  Cache with split lifetimes: {:?}", storage);

    // VARIANCE-SPECIFIC DEBUGGING STRATEGY:
    // 1. Is &mut involved? → Remember: &mut T is INVARIANT over T (lifetime is locked)
    // 2. Same lifetime used in &mut AND inner type? → VARIANCE CONFLICT! Split into 'a, 'b
    // 3. Can't split? → Consider owning the data with .clone() or .to_owned()
    println!("  Fix: split &'a mut Vec<&'a str> into &'a mut Vec<&'b str>");
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    println!("=== Covariance: Lifetimes ===");
    covariance_lifetimes();

    println!("\n=== Covariance: Containers ===");
    covariance_containers();

    println!("\n=== Contravariance: Function Parameters ===");
    contravariance_functions();

    println!("\n=== Invariance: &mut T (Memory Safety) ===");
    invariance_mut_refs();

    println!("\n=== PhantomData: Manual Variance Control ===");
    phantom_data_variance();

    println!("\n=== Covariance: Slices and Arrays ===");
    covariance_slices_and_arrays();

    println!("\n=== Struct Variance Composition ===");
    struct_variance_composition();

    println!("\n=== Trait Objects: Lifetime and Type Variance ===");
    trait_object_variance();

    println!("\n=== Common Error: The Same-Lifetime Trap ===");
    common_lifetime_error_and_fix();

    println!("\n✓ All variance examples passed!");
}

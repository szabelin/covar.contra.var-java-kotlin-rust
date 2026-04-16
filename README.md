# Variance, Contravariance & Covariance: Java, Kotlin, Rust

> **A practical guide for developers who want to finally understand generic variance — starting from where it all went wrong (Java), seeing how it was fixed beautifully (Kotlin), and then exploring how Rust does something entirely different with the same ideas.**

---

## How to Read This Guide

This guide is structured as a journey through three languages, in a very deliberate order:

1. **Java** — where generics and variance were first bolted onto the language. Java is where the *problem* originates. Arrays were made covariant (a mistake that causes runtime crashes), and then generics were added with wildcards (`? extends`, `? super`) that work but are famously ugly and confusing. We start here because understanding *what went wrong* is the fastest way to understand *what variance is*.

2. **Kotlin** — where the same JVM generics system was given a beautiful, clean syntax. Kotlin's `out` and `in` keywords are so intuitive that they make variance feel obvious. Once you see Kotlin's approach, you'll wonder why Java made it so hard. If you already know Java generics, Kotlin will feel like the "aha!" moment.

3. **Rust** *(The Frontier)* — where variance exists but works in a completely different way. Rust has no class inheritance. There's no `Cat extends Animal`. Instead, variance is about *lifetimes* — and the compiler figures it all out automatically. This section is your next challenge after you're comfortable with Java and Kotlin.

**Recommendation:** Read Java and Kotlin first. Make sure you're comfortable with those. Then tackle Rust — it'll make much more sense with that foundation.

---

## Table of Contents

- [Part 0: The Analogy (No Code)](#part-0-the-analogy-no-code)
- [Part 1: Java — Where It All Started (And What Went Wrong)](#part-1-java--where-it-all-started-and-what-went-wrong)
  - [Java's Original Sin: Covariant Arrays](#javas-original-sin-covariant-arrays)
  - [Java Generics: Invariant By Default](#java-generics-invariant-by-default)
  - [Wildcards: Java's Ugly but Correct Fix](#wildcards-javas-ugly-but-correct-fix)
  - [PECS: The Rule That Makes It Click](#pecs-the-rule-that-makes-it-click)
- [Part 2: Kotlin — The Elegant Solution](#part-2-kotlin--the-elegant-solution)
  - [out = Covariant = "I Only Produce T"](#out--covariant--i-only-produce-t)
  - [in = Contravariant = "I Only Consume T"](#in--contravariant--i-only-consume-t)
  - [No Modifier = Invariant = "I Do Both"](#no-modifier--invariant--i-do-both)
  - [Declaration-Site vs Use-Site: Why Kotlin Is Cleaner](#declaration-site-vs-use-site-why-kotlin-is-cleaner)
  - [Kotlin's Standard Library Got It Right](#kotlins-standard-library-got-it-right)
- [Part 3: Rust — The Frontier (Your Next Challenge)](#part-3-rust--the-frontier-your-next-challenge)
  - [The Big Twist: Lifetimes, Not Inheritance](#the-big-twist-lifetimes-not-inheritance)
  - [Covariance in Rust](#covariance-in-rust)
  - [Contravariance in Rust](#contravariance-in-rust)
  - [Invariance in Rust: Why &mut T Is the Key to Everything](#invariance-in-rust-why-mut-t-is-the-key-to-everything)
  - [The Complete Rust Variance Table](#the-complete-rust-variance-table)
  - [PhantomData: Manual Variance Control](#phantomdata-manual-variance-control)
  - [How Struct Variance Is Determined](#how-struct-variance-is-determined)
  - [Trait Objects and Variance](#trait-objects-and-variance)
  - [Common Lifetime Errors Caused by Variance](#common-lifetime-errors-caused-by-variance)
- [Cheat Sheet: All Three Languages Side by Side](#cheat-sheet-all-three-languages-side-by-side)
- [Further Reading](#further-reading)

---

## Part 0: The Analogy (No Code)

Before we touch any code, let's build an intuition with a story. No programming knowledge required.

Imagine you run an animal shelter. You have **Animals**, **Cats**, and **Dogs**. A Cat is always an Animal. A Dog is always an Animal.

### Scenario A: The Glass Display Window (Read-Only)

You have a glass window where people can **look at** the animals inside, but nobody can put animals in or take them out.

If the window contains cats... is it a window of animals? **Yes.** Anyone looking for "a window of animals" will be satisfied. They see cats, which are animals.

> This is **covariance**. A read-only container of a specific type IS a read-only container of the general type.

### Scenario B: The Donation Drop-Off Box (Write-Only)

You have a box where people can **drop off** animals, but they can't see or take out what's inside.

If someone asks for "a drop-off box for cats," can you give them "a drop-off box for any animal"? **Yes!** A box that accepts any animal will certainly accept cats.

But can you give them "a drop-off box only for dogs"? **No.** They'll try to drop off a cat and it won't work.

> This is **contravariance**. The relationship flips. A consumer of the *broader* type can substitute for a consumer of the *narrower* type.

### Scenario C: The Shelter Cage (Read AND Write)

A cage where you can **put animals in AND take them out**.

If you have a cage of cats, can you treat it as a cage of animals?
- **Taking out**: You get a cat. That's an animal. Fine. ✅
- **Putting in**: Someone puts in a dog (it's an animal!). Now your cage of cats has a dog in it. 💥

> This is **invariance**. When you can both read and write, you can't substitute at all.

**These three scenarios are the entire concept of variance.** Everything in this guide is just how Java, Kotlin, and Rust implement these three rules.

### Why Does This Matter?

Variance shows up every time you design or use a generic API. Get it wrong and you'll either get runtime crashes (Java arrays), confusing compiler errors (Java wildcards), or unnecessarily restrictive APIs that force callers to copy data. Get it right and your APIs are flexible, type-safe, and intuitive. Understanding variance is the difference between fighting the compiler and having it work for you.

---

## Part 1: Java — Where It All Started (And What Went Wrong)

Java is where most developers first encounter variance — usually through a confusing compiler error or, worse, a runtime crash. Java's history with variance is messy, and that messiness is actually what makes it a great teaching tool. Let's walk through it.

### Java's Original Sin: Covariant Arrays

When Java was first designed in the mid-1990s, it didn't have generics. It only had arrays. And the designers made a fateful decision: **arrays are covariant**.

```java
// This compiles. This is the mistake.
Animal[] animals = new Cat[3]; // ✅ Cat[] is treated as Animal[]
```

This looks fine at first glance. A cat array IS an animal array, right? Well...

```java
Animal[] animals = new Cat[3];
animals[0] = new Dog(); // 💥 ArrayStoreException at RUNTIME!
```

The compiler said this was fine. The JVM said "absolutely not" — at runtime. You just tried to put a Dog into an array that was actually a Cat array under the hood.

**This is the fundamental problem with covariance on mutable containers.** Reads are safe (taking a Cat out and calling it an Animal is fine), but writes are dangerous (putting a Dog into what's actually a Cat array is not fine).

Why did Java do this? Because without generics, you couldn't write a method like `sort(Object[] arr)` that works on any array. Covariant arrays were a pragmatic compromise. But it's a compromise that moves a compile-time error to a runtime error — and that's always a downgrade.

### Java Generics: Invariant By Default

When generics were added in Java 5 (2004), the designers learned from the array mistake. **Generics are invariant by default.**

```java
List<Cat> cats = new ArrayList<>();
cats.add(new Cat());

// INVARIANT — none of these compile:
List<Animal> animals = cats;   // ❌ Compile error
List<Object> objects = cats;   // ❌ Compile error
```

This is correct! If this were allowed, you could do:

```java
List<Animal> animals = cats;    // hypothetically allowed
animals.add(new Dog());         // adds a Dog to a Cat list!
Cat c = cats.get(0);            // 💥 ClassCastException!
```

The exact same problem as arrays, just caught at compile time instead of runtime. The invariant default is the *safe* choice.

But now you have a different problem: you can't write a method that accepts `List<Animal>` and pass it a `List<Cat>`. That's too restrictive. You need a way to opt into covariance or contravariance when it's safe.

### Wildcards: Java's Ugly but Correct Fix

Java's solution is **wildcards**. They work, but the syntax is... not pretty.

#### `? extends T` — Covariant (Read-Only View)

```java
List<Cat> cats = List.of(new Cat(), new Cat());

// "A list of some type that extends Animal"
List<? extends Animal> animals = cats; // ✅ Covariant!

// You can READ (the ? is some subtype of Animal, so you always get an Animal):
Animal a = animals.get(0); // ✅

// You CANNOT WRITE (the ? might be Cat, Dog, or Hamster — compiler doesn't know):
animals.add(new Cat());    // ❌ Compile error
animals.add(new Animal()); // ❌ Compile error
```

The wildcard `? extends Animal` means "I promise to only READ from this list." And if you only read, covariance is safe — you always get at least an Animal.

#### `? super T` — Contravariant (Write-Only View)

```java
List<Animal> animals = new ArrayList<>();

// "A list of some type that is a supertype of Cat"
List<? super Cat> catAcceptor = animals; // ✅ Contravariant!

// You can WRITE Cats (whatever the ? is, it's a supertype of Cat, so Cat fits):
catAcceptor.add(new Cat()); // ✅

// You CANNOT READ as Cat (the ? might be Animal or Object):
Cat c = catAcceptor.get(0); // ❌ Compile error
Object o = catAcceptor.get(0); // ✅ Only guaranteed to be Object
```

#### The Problem With Java's Approach

It works, but look at the syntax. `List<? extends Animal>` and `List<? super Cat>` are not intuitive. Every time you use them, you have to stop and think "extends means I read, super means I write... wait, or is it the other way?"

That's why Josh Bloch invented the PECS mnemonic.

### PECS: The Rule That Makes It Click

**PECS = Producer Extends, Consumer Super**

- If the generic type **produces** values (you read from it) → use `? extends T`
- If the generic type **consumes** values (you write to it) → use `? super T`
- If it does **both** → use plain `T` (invariant, no wildcard)

```java
// PRODUCER — you read Animals out of it
void printAll(List<? extends Animal> animals) {
    for (Animal a : animals) {
        System.out.println(a.name());
    }
}

// CONSUMER — you write Cats into it
void addCats(List<? super Cat> list) {
    list.add(new Cat());
    list.add(new Cat());
}

// BOTH — no wildcard possible
void swap(List<Animal> list, int i, int j) {
    Animal temp = list.get(i);  // read
    list.set(i, list.get(j));   // write
    list.set(j, temp);          // write
}
```

PECS is the single most useful thing to remember from Java generics. And as you'll see, it maps directly to Kotlin's `out`/`in` and even to Rust's variance rules.

#### A Note on Type Erasure

Java erases generic type information at runtime — a `List<Cat>` becomes just a `List` in the compiled bytecode. This means variance is purely a **compile-time** concept in Java. The compiler enforces wildcard rules, but the JVM has no idea whether a `List` was originally a `List<Cat>` or `List<Animal>`. This is why Java's arrays (which *do* retain type information at runtime) can throw `ArrayStoreException`, while generics catch the equivalent errors at compile time.

---

## Part 2: Kotlin — The Elegant Solution

Kotlin looked at Java's wildcards and said: "This is correct, but it can be so much cleaner." And they were right.

### `out` = Covariant = "I Only Produce T"

Kotlin replaces `? extends T` with the keyword **`out`**. Read it as: "T only comes *out* of this type."

```kotlin
open class Animal { open fun name() = "Animal" }
class Cat : Animal() { override fun name() = "Cat" }
class Dog : Animal() { override fun name() = "Dog" }

// 'out' means: T is only used in OUTPUT positions (return types)
class Cage<out T>(private val animal: T) {
    fun look(): T = animal      // ✅ T comes OUT — return type
    // fun put(a: T) { }        // ❌ Compile error! T can't go IN
}

// Now covariance just works:
val catCage: Cage<Cat> = Cage(Cat())
val animalCage: Cage<Animal> = catCage // ✅ No wildcards needed!

println(animalCage.look().name()) // prints "Cat"
```

Compare this to Java:

```
Java:   Cage<? extends Animal> animalCage = catCage;
Kotlin: val animalCage: Cage<Animal> = catCage
```

Same safety guarantees. Half the syntax. The `out` keyword on the *class declaration* means you never have to think about it at each usage site.

### `in` = Contravariant = "I Only Consume T"

Kotlin replaces `? super T` with the keyword **`in`**. Read it as: "T only goes *in* to this type."

```kotlin
// 'in' means: T is only used in INPUT positions (parameters)
interface Handler<in T> {
    fun handle(t: T)         // ✅ T goes IN — parameter
    // fun produce(): T      // ❌ Compile error! T can't come OUT
}

val animalHandler = object : Handler<Animal> {
    override fun handle(t: Animal) {
        println("Handling ${t.name()}")
    }
}

// Contravariant: Handler<Animal> IS-A Handler<Cat>
// (A handler for ALL animals can certainly handle cats)
val catHandler: Handler<Cat> = animalHandler // ✅ Reversed!
catHandler.handle(Cat()) // prints "Handling Cat"
```

This is the same as Java's `Comparator<? super Cat>` accepting a `Comparator<Animal>`, but declared once on the interface instead of at every usage.

### No Modifier = Invariant = "I Do Both"

When a type parameter has no `in` or `out`, it's invariant. This is the safe default for types that both read and write.

```kotlin
// MutableList<T> — no modifier, invariant
val cats: MutableList<Cat> = mutableListOf(Cat())
// val animals: MutableList<Animal> = cats // ❌ Won't compile!

// WHY? Because MutableList lets you both add() and get().
// If this were allowed:
//   animals.add(Dog())       // adds a Dog...
//   val cat: Cat = cats[0]   // but cats expects a Cat! 💥
// Same problem as Java's covariant arrays, prevented at compile time.
```

### Declaration-Site vs Use-Site: Why Kotlin Is Cleaner

The key innovation in Kotlin is **declaration-site variance**. You put `out` or `in` on the *class/interface definition*, not at every place you use it.

```kotlin
// DECLARATION-SITE: decided ONCE, applies everywhere
interface Source<out T> {        // ← declared here
    fun produce(): T
}

val catSource: Source<Cat> = /* ... */
val animalSource: Source<Animal> = catSource // ✅ Always works

// VS Java's USE-SITE: decided at EVERY usage
// Source<? extends Animal> animalSource = catSource; // ← repeated everywhere
```

Kotlin *also* supports use-site variance (called "type projections") for cases where the class itself is invariant but you want a covariant or contravariant *view* of it:

```kotlin
// MutableList is invariant, but you can project it:
fun copy(
    from: MutableList<out Animal>,  // read-only view (covariant)
    to: MutableList<in Animal>      // write-only view (contravariant)
) {
    for (a in from) to.add(a)
}
```

This is the same as Java's wildcards, but you rarely need it because most Kotlin standard library types already have the right variance declared.

### Kotlin's Standard Library Got It Right

Kotlin's standard library uses declaration-site variance throughout. This means the *right thing* happens automatically:

```kotlin
// List<out E> — covariant, read-only
val cats: List<Cat> = listOf(Cat())
val animals: List<Animal> = cats // ✅ Just works!

// MutableList<E> — invariant, mutable
val mutCats: MutableList<Cat> = mutableListOf(Cat())
// val mutAnimals: MutableList<Animal> = mutCats // ❌ Correctly prevented

// Comparable<in T> — contravariant
// A Comparable<Animal> can compare Cats
val animalComparable: Comparable<Animal> = /* ... */
val catComparable: Comparable<Cat> = animalComparable // ✅

// Function types use variance too:
// (Animal) -> String  IS-A  (Cat) -> String  (contravariant in parameter)
// () -> Cat  IS-A  () -> Animal  (covariant in return)
```

Compare this to Java, where you'd need wildcards at every usage site. Kotlin's approach means you understand variance *once* (when reading the class declaration) and then it just works everywhere.

**The mental model is simple:**
- `out` = "only comes out" = safe to treat as broader type = covariant
- `in` = "only goes in" = safe to treat as narrower type = contravariant
- nothing = "goes both ways" = no substitution = invariant

If you've understood Java's PECS, Kotlin's `out`/`in` is the same thing with cleaner syntax. And if you understand Kotlin's `out`/`in`, you have the perfect foundation for understanding Rust.

---

## Part 3: Rust — The Frontier (Your Next Challenge)

> **Prerequisites:** Make sure you're comfortable with Java's PECS or Kotlin's `out`/`in` before reading this section. The concepts are identical — Rust just applies them to something very different.

Everything you learned about variance in Java and Kotlin still applies in Rust. The same three rules. The same intuition about reading vs writing. But there's one fundamental difference that changes everything.

### The Big Twist: Lifetimes, Not Inheritance

In Java and Kotlin, subtyping comes from **inheritance**:

```kotlin
open class Animal
class Cat : Animal()
// Cat is a subtype of Animal
```

Rust has no class-based inheritance. There is no `Cat extends Animal`. Rust uses traits for polymorphism, but trait implementations don't create subtype relationships the way class inheritance does in Java or Kotlin. Instead, Rust's subtyping is about **lifetimes**:

```rust
// 'long: 'short means 'long outlives 'short
// Therefore 'long is a SUBTYPE of 'short
```

Wait — a *longer* lifetime is a subtype of a *shorter* one?

**Yes.** Remember, "subtype" means "can be used in place of." A reference that lives for 10 seconds can be used where a reference that lives for 5 seconds is expected. The longer-lived reference is *more specific* (it makes a stronger guarantee), so it's the subtype.

Think of it like contracts: a loan guarantee valid for 10 years is strictly better than one valid for 5 years. Anyone who needs a "5-year guarantee" will accept a "10-year guarantee" — because it covers everything the 5-year one does and more. That's why the longer (more specific) lifetime is the subtype.

Think of it this way:

| In Java/Kotlin | In Rust |
|---|---|
| `Animal` (general, supertype) | `'short` (shorter lifetime, less specific) |
| `Cat` (specific, subtype) | `'long` (longer lifetime, more specific) |
| `Cat extends Animal` | `'long: 'short` |

Once you make this mental substitution, **every variance rule you already know applies identically in Rust.** The only difference is what "subtype" means.

### Covariance in Rust

Just like Kotlin's `List<out E>`, Rust has types that are covariant — you can substitute a more specific (longer-lived) type where a less specific (shorter-lived) one is expected.

```rust
// &'a T is covariant over 'a.
// If 'long: 'short, then &'long str can be used as &'short str.

fn example<'long, 'short>(x: &'long str)
where
    'long: 'short,
{
    let y: &'short str = x; // ✅ Covariance!
    // A reference that lives longer can be "downgraded"
    // to one that lives shorter. Totally safe — the data
    // will still be alive when the shorter reference expires.
}
```

Covariant types in Rust: `&'a T`, `Box<T>`, `Vec<T>`, `Option<T>`, `[T]`, `[T; n]`, `*const T`.

These are all "producers" — you read values out of them. Same as Kotlin's `out`, same as Java's `? extends`.

### Contravariance in Rust

In Kotlin, `in T` means "I only consume T." In Rust, contravariance primarily appears in **function parameter types** (and in `PhantomData<fn(T)>`, which derives from this).

```rust
// fn(T) is contravariant over T.
// If 'long: 'short, then fn(&'short str) <: fn(&'long str)
// The direction REVERSES!

fn handle_any(s: &str) {
    println!("{}", s);
}

fn needs_static_handler(f: fn(&'static str)) {
    f("this string lives forever");
}

fn demo() {
    // handle_any accepts ANY lifetime (short or long).
    // needs_static_handler wants a function for 'static refs.
    // handle_any is "wider" — it can certainly handle 'static.
    needs_static_handler(handle_any); // ✅ Contravariance!
}
```

> **Note:** Under the hood, `handle_any` has type `for<'a> fn(&'a str)` (a higher-ranked type due to lifetime elision). The coercion to `fn(&'static str)` works through higher-ranked subtyping — the universally quantified lifetime can be instantiated to `'static`. This is contravariance applied through HRTB elaboration rather than between two concrete lifetimes.

This is the same intuition as Kotlin's `in` or Java's `? super`: a function that handles the broader type can substitute for one that handles the narrower type. A vet who treats all animals can treat your cat.

### Invariance in Rust: Why `&mut T` Is the Key to Everything

This is the most important variance rule in Rust. It's why Rust is memory-safe without a garbage collector.

`&mut T` is **invariant** over `T`. You cannot substitute `&mut &'long str` for `&mut &'short str`, even though `&'long str` is a subtype of `&'short str`.

Why? The exact same reason as Java's invariant generics and Kotlin's `MutableList`. **If you can both read and write, substitution is unsafe.**

Here's the proof — imagine `&mut T` were covariant:

```rust
fn this_would_be_a_disaster() {
    let mut forever: &str = "I live forever"; // 'static lifetime

    let smuggled: &str = {
        let mut ref_to_forever: &mut &str = &mut forever;

        // IF &mut T were covariant, we could do this:
        // Treat &mut &'static str as &mut &'short str
        // Then write a short-lived reference through it:

        let temp = String::from("I die soon");
        // *ref_to_forever = &temp;  // Write a short-lived ref!

        forever // Still claims to be 'static!
    };
    // temp is dropped here.
    // smuggled now points to freed memory.
    // println!("{}", smuggled); // 💥 USE-AFTER-FREE!
}
```

Because `&mut T` is invariant, the compiler prevents this at compile time. **Zero runtime cost. No garbage collector. No runtime checks.** This is the core of Rust's memory safety guarantee.

Other invariant types: `Cell<T>`, `RefCell<T>`, `UnsafeCell<T>`, `*mut T`. They're all invariant because they all allow mutation.

### The Complete Rust Variance Table

| Type | Variance over `'a` | Variance over `T` |
|---|---|---|
| `&'a T` | covariant | covariant |
| `&'a mut T` | covariant | **invariant** |
| `Box<T>` | — | covariant |
| `Vec<T>` | — | covariant |
| `Option<T>` | — | covariant |
| `[T]` and `[T; n]` | — | covariant |
| `*const T` | — | covariant |
| `*mut T` | — | **invariant** |
| `Cell<T>` | — | **invariant** |
| `RefCell<T>` | — | **invariant** |
| `UnsafeCell<T>` | — | **invariant** |
| `fn(T) -> U` | — | **contra** (T) / co (U) |
| `dyn Trait<T> + 'a` | covariant | **invariant** |

**The rules are the same as Java/Kotlin, just expressed differently:**

| Rule | Java | Kotlin | Rust |
|---|---|---|---|
| Read-only → covariant | `? extends T` | `out T` | Automatic (compiler sees read-only usage) |
| Write-only → contravariant | `? super T` | `in T` | Automatic (only in `fn(T)` params) |
| Read + write → invariant | Plain `T` | No modifier | Automatic (compiler sees mutation) |

The big difference: **in Java you write wildcards, in Kotlin you write `out`/`in`, in Rust the compiler figures it out.** You rarely need to declare variance explicitly in Rust — the compiler derives it from how your type parameters are used in struct fields (though `PhantomData` is a form of explicit variance annotation, as shown below).

### PhantomData: Manual Variance Control

Sometimes you have a type parameter that doesn't appear in any field (common with raw pointers and FFI). The compiler doesn't know what variance to give it. `PhantomData` is how you tell it.

```rust
use std::marker::PhantomData;

// T doesn't appear in any field — compiler doesn't know the variance
struct JsonKey<V> {
    raw: String,
    _type: PhantomData<V>,  // "Pretend I hold a V" → covariant
}

// Different PhantomData types → different variance:
PhantomData<T>            // covariant     (like Kotlin's out)
PhantomData<fn(T)>        // contravariant (like Kotlin's in)
PhantomData<fn(T) -> T>   // invariant     (like Kotlin's no modifier)
```

You'll mainly encounter this when writing unsafe code or reading library internals. For everyday Rust, you rarely need `PhantomData`.

### How Struct Variance Is Determined

When you write a custom struct, the compiler derives its variance from the fields. The rule is simple:

```rust
use std::cell::Cell;

struct MyType<'a, A, B, C, D> {
    a: &'a A,        // covariant over both 'a and A
    b: &'a mut B,    // covariant over 'a, invariant over B
    c: Cell<C>,      // invariant over C
    d: fn(D) -> u32, // contravariant over D
}
// Result: MyType is covariant over 'a, but INVARIANT over A, B, C, and D
```

The rules:
- **All covariant uses** → covariant over that parameter
- **All contravariant uses** → contravariant over that parameter
- **Mixed uses** → **invariant** (invariance wins all conflicts)

If a type parameter appears in even one invariant field (like `Cell<T>` or `&mut T`), the entire struct becomes invariant over that parameter. This is how the compiler ensures your custom generic types are safe — you never declare variance explicitly, it's computed from your fields.

### Trait Objects and Variance

While traits don't create subtype hierarchies the way classes do, trait objects interact with variance through their lifetime bounds:

```rust
// Box<dyn Trait + 'a> is covariant over 'a.
// A Box<dyn Display + 'static> can be used where Box<dyn Display + 'a> is expected,
// because 'static outlives any 'a.

fn print_it(item: Box<dyn std::fmt::Display + '_>) {
    println!("{}", item);
}

fn demo() {
    let s = String::from("hello");
    let boxed: Box<dyn std::fmt::Display + 'static> = Box::new("static str");
    print_it(boxed); // ✅ 'static coerces to shorter lifetime
}
```

This is the same covariance you see with `&'a T` — longer lifetime substitutes for shorter.

However, `dyn Trait<T>` is **invariant** over `T`. This means `Box<dyn Trait<Cat>>` is NOT a subtype of `Box<dyn Trait<Animal>>`, even if `Cat` were a subtype of `Animal`. The trait object's vtable is fixed at creation time — the compiler can't safely assume type parameter substitutability across trait objects.

> **Note:** If you're writing unsafe code or FFI bindings, variance isn't always automatic. Raw pointers and `unsafe` blocks can create situations where the compiler's variance assumptions don't hold, requiring careful manual reasoning beyond just `PhantomData`.

### Common Lifetime Errors Caused by Variance

These are the real-world situations where variance will bite you in Rust.

#### The "Same Lifetime Everywhere" Trap

```rust
// ❌ BAD: Same lifetime for the mutable borrow AND the contents
struct Cache<'a> {
    data: &'a mut Vec<&'a str>,
}
// &'a mut makes the Vec invariant over 'a.
// But Vec<&'a str> needs 'a to be covariant.
// Same 'a, conflicting requirements → weird borrow errors.

// ✅ GOOD: Separate lifetimes
struct Cache<'a, 'b> {
    data: &'a mut Vec<&'b str>,
}
// Now 'a (the mutable borrow) and 'b (the string references)
// are independent. No conflict.
```

#### General Debugging Strategy

When you get a confusing lifetime error:

1. **Is `&mut` involved?** If yes, remember it's invariant over `T`. The inner type's lifetime gets locked.
2. **Is the same lifetime used twice?** Using `'a` in both `&'a mut` and `&'a T` can create conflicting requirements. Try splitting into `'a` and `'b`.
3. **Consider owning the data.** Sometimes `.clone()` or `.to_owned()` is the right fix instead of fighting lifetimes.

### Rust Variance at a Glance

Everything in the Rust section, in one table. This is verified against the official [Rust Reference](https://doc.rust-lang.org/reference/subtyping.html) and [Rustonomicon](https://doc.rust-lang.org/nomicon/subtyping.html).

| Type | Variance over `'a` | Variance over `T` | Why |
|---|---|---|---|
| `&'a T` | covariant | covariant | Read-only — safe to narrow |
| `&'a mut T` | covariant | **invariant** | Mutation — must prevent lifetime smuggling |
| `Box<T>` | — | covariant | Owned, no shared mutation |
| `Vec<T>` | — | covariant | Owned, no shared mutation |
| `Option<T>` | — | covariant | Wrapper, no mutation |
| `[T]` / `[T; n]` | — | covariant | Sequences, same as `Vec<T>` |
| `*const T` | — | covariant | Read-only raw pointer |
| `*mut T` | — | **invariant** | Mutable raw pointer |
| `Cell<T>` | — | **invariant** | Interior mutability |
| `RefCell<T>` | — | **invariant** | Interior mutability |
| `UnsafeCell<T>` | — | **invariant** | Primitive interior mutability |
| `fn(T) -> U` | — | **contra** (T) / co (U) | Parameters consumed, return produced |
| `dyn Trait<T> + 'a` | covariant | **invariant** | Lifetime can flex, type param is fixed |

**How variance is determined for your types:**

| Your struct's field usage | Result | Analogy |
|---|---|---|
| All uses are covariant | **Covariant** | Like Kotlin's `out` |
| All uses are contravariant | **Contravariant** | Like Kotlin's `in` |
| Mixed, or any invariant field | **Invariant** | Like Kotlin's no modifier |

**PhantomData patterns for manual control:**

| PhantomData type | Variance | Kotlin equivalent |
|---|---|---|
| `PhantomData<T>` | Covariant | `out T` |
| `PhantomData<fn(T)>` | Contravariant | `in T` |
| `PhantomData<fn(T) -> T>` | Invariant | `T` (no modifier) |

> **Beyond this guide:** Rust's variance system also includes advanced topics like higher-ranked lifetime subtyping (`for<'a>`) and `NonNull<T>` covariance patterns. These are primarily relevant for unsafe code and collection implementations. See the sources below for the complete specification.

*All Rust examples compile on stable Rust (2021 edition and later). No nightly features required. See `variance_examples.rs` to run them yourself.*

### Sources

This guide's Rust section was verified against the following authoritative references:

- [**The Rustonomicon: Subtyping and Variance**](https://doc.rust-lang.org/nomicon/subtyping.html) — The official unsafe Rust guide's chapter on variance. Covers the variance table, subtyping rules, and the `&mut T` invariance proof.
- [**The Rust Reference: Subtyping and Variance**](https://doc.rust-lang.org/reference/subtyping.html) — The language specification's variance table, including `[T]`, `[T; n]`, `dyn Trait<T> + 'a`, and higher-ranked lifetime rules.
- [**RFC 0738: Variance**](https://rust-lang.github.io/rfcs/0738-variance.html) — The original RFC that defined Rust's variance inference system and struct composition rules.
- [**Learning Rust With Entirely Too Many Linked Lists: Variance**](https://rust-unofficial.github.io/too-many-lists/sixth-variance.html) — Practical variance examples including `NonNull<T>` covariance patterns and `PhantomData` usage in collection implementations.
- [**"Blindsided by Rust's Subtyping and Variance" (NullDeref)**](https://nullderef.com/blog/rust-variance/) — Real-world case study of variance-induced bugs, trait-induced invariance, and debugging strategies.
- [**"Variance: Best Perspective of Understanding Lifetime in Rust" (DEV Community)**](https://dev.to/arichy/variance-best-perspective-of-understanding-lifetime-in-rust-m84) — Variance framed through read/write data flow, with practical lifetime examples.

---

## Cheat Sheet: All Three Languages Side by Side

### The Three Types of Variance

| Variance | Meaning | Java | Kotlin | Rust |
|---|---|---|---|---|
| **Covariant** | Cat-container → Animal-container | `? extends T` | `out T` | `&T`, `Box<T>` (auto) |
| **Contravariant** | Animal-handler → Cat-handler | `? super T` | `in T` | `fn(T)` (auto) |
| **Invariant** | No relationship | `T` (default) | `T` (default) | `&mut T`, `Cell<T>` (auto) |

### When Is Each Safe?

| Access Pattern | Variance | Why |
|---|---|---|
| **Read-only** (produce values) | Covariant | Narrowing what comes out is always safe |
| **Write-only** (consume values) | Contravariant | Widening what goes in is always safe |
| **Read AND write** | Invariant | Both directions are unsafe, so no substitution |

### PECS Across Languages

| Mnemonic | Java | Kotlin | Rust |
|---|---|---|---|
| **P**roducer **E**xtends | `List<? extends Animal>` | `List<out Animal>` | `Vec<&'short str>` accepting `Vec<&'long str>` |
| **C**onsumer **S**uper | `Comparator<? super Cat>` | `Handler<in Cat>` | `fn(&'short str)` used as `fn(&'long str)` |

### What "Subtype" Means in Each Language

| Language | Subtype means... | Example |
|---|---|---|
| Java | Inherits from | `Cat extends Animal` |
| Kotlin | Inherits from | `Cat : Animal()` |
| Rust | **Outlives** | `'long: 'short` (longer lifetime is the subtype) |

### Key Mistakes to Avoid

| Language | Classic Mistake | What Happens |
|---|---|---|
| Java | Covariant arrays: `Animal[] a = new Cat[3]` | 💥 Runtime `ArrayStoreException` |
| Java | Forgetting PECS: using `List<Animal>` where `List<? extends Animal>` is needed | ❌ Compile error (but misleading) |
| Kotlin | Trying to add `out` to a type that reads AND writes | ❌ Compile error (Kotlin catches it) |
| Rust | Using the same lifetime for `&'a mut` and inner `&'a` references | ❌ Confusing borrow checker errors |

---

## Further Reading

**Java:**
- Effective Java by Joshua Bloch — Item 31: "Use bounded wildcards to increase API flexibility"
- [Java Generics FAQ](http://www.angelikalanger.com/GenericsFAQ/JavaGenericsFAQ.html) by Angelika Langer

**Kotlin:**
- [Kotlin Docs: Generics — Variance](https://kotlinlang.org/docs/generics.html)
- Kotlin in Action by Dmitry Jemerov & Svetlana Isakova — Chapter 9

**Rust:**
- [The Rustonomicon: Subtyping and Variance](https://doc.rust-lang.org/nomicon/subtyping.html)
- [Rust Reference: Variance](https://doc.rust-lang.org/reference/subtyping.html)

---

*This guide is structured for developers coming from Java or Kotlin who want to understand Rust's approach to variance. Start with what you know, then explore the frontier.*

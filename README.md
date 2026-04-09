# Variance, Contravariance & Covariance: Java, Kotlin, Rust

> **A practical guide for developers who want to finally understand generic variance — starting from where it all went wrong (Java), seeing how it was fixed beautifully (Kotlin), and then exploring how Rust does something entirely different with the same ideas.**

---

## How to Read This Guide

This guide is structured as a journey through two languages, in a deliberate order:

1. **Java** — where generics and variance were first bolted onto the language. Java is where the *problem* originates. Arrays were made covariant (a mistake that causes runtime crashes), and then generics were added with wildcards (`? extends`, `? super`) that work but are famously ugly and confusing. We start here because understanding *what went wrong* is the fastest way to understand *what variance is*.

2. **Kotlin** — where the same JVM generics system was given a beautiful, clean syntax. Kotlin's `out` and `in` keywords are so intuitive that they make variance feel obvious. Once you see Kotlin's approach, you'll wonder why Java made it so hard. If you already know Java generics, Kotlin will feel like the "aha!" moment.

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
- [Cheat Sheet: Java & Kotlin Side by Side](#cheat-sheet-java--kotlin-side-by-side)
- [Part 3: Rust — The Frontier (Coming Soon)](#part-3-rust--the-frontier-coming-soon)
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

**These three scenarios are the entire concept of variance.** Everything in this guide is just how Java and Kotlin implement these three rules.

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

PECS is the single most useful thing to remember from Java generics. And as you'll see, it maps directly to Kotlin's `out`/`in`.

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

If you've understood Java's PECS, Kotlin's `out`/`in` is the same thing with cleaner syntax.

---

## Cheat Sheet: Java & Kotlin Side by Side

### The Three Types of Variance

| Variance | Meaning | Java | Kotlin |
|---|---|---|---|
| **Covariant** | Cat-container → Animal-container | `? extends T` | `out T` |
| **Contravariant** | Animal-handler → Cat-handler | `? super T` | `in T` |
| **Invariant** | No relationship | `T` (default) | `T` (default) |

### When Is Each Safe?

| Access Pattern | Variance | Why |
|---|---|---|
| **Read-only** (produce values) | Covariant | Narrowing what comes out is always safe |
| **Write-only** (consume values) | Contravariant | Widening what goes in is always safe |
| **Read AND write** | Invariant | Both directions are unsafe, so no substitution |

### PECS Across Languages

| Mnemonic | Java | Kotlin |
|---|---|---|
| **P**roducer **E**xtends | `List<? extends Animal>` | `List<out Animal>` |
| **C**onsumer **S**uper | `Comparator<? super Cat>` | `Handler<in Cat>` |

### Key Mistakes to Avoid

| Language | Classic Mistake | What Happens |
|---|---|---|
| Java | Covariant arrays: `Animal[] a = new Cat[3]` | 💥 Runtime `ArrayStoreException` |
| Java | Forgetting PECS: using `List<Animal>` where `List<? extends Animal>` is needed | ❌ Compile error (but misleading) |
| Kotlin | Trying to add `out` to a type that reads AND writes | ❌ Compile error (Kotlin catches it) |

---

## Part 3: Rust — The Frontier (Coming Soon)

Rust applies variance to **lifetimes** instead of class inheritance — and the compiler figures it all out automatically. No `out`, no `in`, no wildcards.

Topics coming:
- Lifetimes as subtyping (`'long: 'short`)
- Why `&mut T` is invariant (and why that's the key to memory safety)
- `PhantomData` for manual variance control
- The complete Rust variance table
- Common lifetime errors caused by variance

Stay tuned.

---

## Further Reading

**Java:**
- Effective Java by Joshua Bloch — Item 31: "Use bounded wildcards to increase API flexibility"
- [Java Generics FAQ](http://www.angelikalanger.com/GenericsFAQ/JavaGenericsFAQ.html) by Angelika Langer

**Kotlin:**
- [Kotlin Docs: Generics — Variance](https://kotlinlang.org/docs/generics.html)
- Kotlin in Action by Dmitry Jemerov & Svetlana Isakova — Chapter 9

---

*This guide is structured for developers coming from Java or Kotlin who want to understand generic variance. Rust coverage is coming soon.*

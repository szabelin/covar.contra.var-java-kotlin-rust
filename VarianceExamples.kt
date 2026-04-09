/**
 * Variance in Kotlin: out (covariant), in (contravariant), invariant default.
 */

open class Animal { open fun name() = "Animal" }
class Cat : Animal() { override fun name() = "Cat" }
class Dog : Animal() { override fun name() = "Dog" }

// --- out = covariant = "I only produce T" ---

class Cage<out T>(private val animal: T) {
    fun look(): T = animal          // ✅ T in output position
    // fun put(a: T) { }            // ❌ compile error — T can't go in
}

// --- in = contravariant = "I only consume T" ---

interface Handler<in T> {
    fun handle(t: T)                // ✅ T in input position
    // fun produce(): T             // ❌ compile error — T can't come out
}

// --- no modifier = invariant = "I do both" ---
// MutableList<T> is invariant — can't assign MutableList<Cat> to MutableList<Animal>

// --- Declaration-site vs use-site ---

interface Source<out T> {
    fun produce(): T
}

// Use-site projection on an invariant type
fun copy(from: MutableList<out Animal>, to: MutableList<in Animal>) {
    for (a in from) to.add(a)
}

fun main() {
    // Covariance: Cage<Cat> IS-A Cage<Animal>
    println("=== out (covariant) ===")
    val catCage: Cage<Cat> = Cage(Cat())
    val animalCage: Cage<Animal> = catCage  // ✅
    println(animalCage.look().name())

    // Contravariance: Handler<Animal> IS-A Handler<Cat>
    println("\n=== in (contravariant) ===")
    val animalHandler = object : Handler<Animal> {
        override fun handle(t: Animal) {
            println("Handling ${t.name()}")
        }
    }
    val catHandler: Handler<Cat> = animalHandler  // ✅ reversed
    catHandler.handle(Cat())

    // Invariance: MutableList<Cat> is NOT MutableList<Animal>
    println("\n=== invariant ===")
    val cats: MutableList<Cat> = mutableListOf(Cat())
    // val animals: MutableList<Animal> = cats  // ❌ won't compile

    // Covariant read-only list
    println("\n=== List<out E> stdlib ===")
    val catList: List<Cat> = listOf(Cat(), Cat())
    val animalList: List<Animal> = catList   // ✅ List is covariant
    println("Animals: ${animalList.size}")
}

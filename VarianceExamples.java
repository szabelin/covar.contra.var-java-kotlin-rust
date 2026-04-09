import java.util.ArrayList;
import java.util.List;

/**
 * Variance in Java: covariant arrays, invariant generics, wildcards, PECS.
 */
public class VarianceExamples {

    static class Animal {
        String name() { return "Animal"; }
    }

    static class Cat extends Animal {
        @Override String name() { return "Cat"; }
    }

    static class Dog extends Animal {
        @Override String name() { return "Dog"; }
    }

    // --- Covariant arrays (Java's mistake) ---

    static void covariantArraysProblem() {
        Animal[] animals = new Cat[3];      // compiles — Cat[] IS-A Animal[]
        animals[0] = new Cat();             // fine
        // animals[1] = new Dog();          // 💥 ArrayStoreException at RUNTIME
    }

    // --- Invariant generics (the fix) ---

    static void invariantGenerics() {
        List<Cat> cats = new ArrayList<>();
        cats.add(new Cat());
        // List<Animal> animals = cats;     // ❌ compile error — invariant
    }

    // --- ? extends T — covariant / producer / read-only ---

    static void printAll(List<? extends Animal> animals) {
        for (Animal a : animals) {
            System.out.println(a.name());   // ✅ reading is safe
        }
        // animals.add(new Cat());          // ❌ writing is blocked
    }

    // --- ? super T — contravariant / consumer / write-only ---

    static void addCats(List<? super Cat> list) {
        list.add(new Cat());                // ✅ writing Cats is safe
        // Cat c = list.get(0);             // ❌ reading as Cat is blocked
        Object o = list.get(0);             // ✅ only guaranteed Object
    }

    // --- Plain T — invariant / read + write ---

    static void swap(List<Animal> list, int i, int j) {
        Animal temp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, temp);
    }

    public static void main(String[] args) {
        // Producer Extends
        System.out.println("=== ? extends (covariant) ===");
        List<Cat> cats = List.of(new Cat(), new Cat());
        printAll(cats);

        // Consumer Super
        System.out.println("\n=== ? super (contravariant) ===");
        List<Animal> animals = new ArrayList<>();
        addCats(animals);
        System.out.println("Size after addCats: " + animals.size());

        // Invariant
        System.out.println("\n=== invariant (read + write) ===");
        List<Animal> mixed = new ArrayList<>(List.of(new Cat(), new Dog()));
        swap(mixed, 0, 1);
        System.out.println("After swap: " + mixed.get(0).name() + ", " + mixed.get(1).name());
    }
}

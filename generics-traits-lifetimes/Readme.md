# Rust Core Concepts: Generics, Traits, Lifetimes

## Generics
- **Definition:** Generics allow you to write flexible and reusable code that can work with multiple data types while keeping type safety.
- **Concepts:**
  - Defined using angle brackets `<T>`.
  - Constraints can be applied with traits (e.g., `T: PartialOrd`).
  - Used in functions, structs, enums, and implementations.
  - Removes duplication by generalizing over types.

---

## Traits
- **Definition:** Traits define shared behavior that types can implement, similar to "interfaces" in other languages.
- **Concepts:**
  - Declared with the `trait` keyword.
  - Implemented for a type using `impl TraitName for Type`.
  - Can include method signatures (with or without default implementations).
  - Enable **polymorphism** in Rust.

---

## Lifetimes
- **Definition:** Lifetimes are a way of telling the Rust compiler how long references are valid, ensuring memory safety without a garbage collector.
- **Concepts:**
  - Written as `'a`, `'b`, etc.
  - They don't change how long a value lives, only how references relate to each other.
  - Required when the compiler cannot infer reference relationships.
  - Common in functions that return references based on input references.

---

# Rust Smart Pointers - Definitions

## Box<T>
A smart pointer that allocates data on the heap. Provides ownership and ensures memory is freed when the Box goes out of scope. Useful for large data or recursive types.

## Deref Trait
Allows a smart pointer to behave like a regular reference using the `*` operator. Enables deref coercion where Rust automatically converts references.

## Drop Trait
Defines custom behavior when a value goes out of scope. Used for cleanup tasks like closing files or freeing resources.

## Rc<T> (Reference Counted)
A smart pointer that enables multiple owners of the same data. Keeps track of the number of references and deallocates memory when the count reaches zero. Immutable by default.

## RefCell<T> (Interior Mutability)
Allows mutation of data even when the RefCell itself is immutable. Enforces Rust's borrowing rules at runtime rather than compile time.

## Reference Cycles
When smart pointers reference each other in a cycle, memory may not be freed automatically, causing leaks. Using `Weak<T>` can prevent cycles by creating non-owning references.

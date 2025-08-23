## Closures
- Anonymous functions that can be stored in variables or passed as arguments.  
- Can capture values from the environment by immutable borrow, mutable borrow, or by taking ownership (`move`).  
- Syntax is flexible and often inferred by the compiler.  

---

## Iterators
- Objects that produce a sequence of values, one at a time.  
- They are lazy, meaning no work is done until consumed.  
- Two main categories:
  - Consuming adaptors → use up the iterator (e.g., `sum`, `collect`).  
  - Iterator adaptors → create new iterators (e.g., `map`, `filter`).  
- Custom iterators can be created by implementing the `Iterator` trait and defining the `next()` method.  

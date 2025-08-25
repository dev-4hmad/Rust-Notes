# Tests in Rust

Rust has a built-in testing framework that helps ensure code works correctly and prevents future bugs.

---

## Types of Tests
1. **Unit Tests**  
   - Small tests for individual functions or modules.  
   - Usually placed inside the same file with `#[cfg(test)]`.

2. **Integration Tests**  
   - Test how different parts of the crate work together.  
   - Placed in the `tests/` directory.

3. **Documentation Tests**  
   - Examples in documentation comments (`///`) that get compiled and run.  
   - Keeps docs and code consistent.

---

## Writing a Test
Use the `#[test]` attribute to mark a function as a test.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(2 + 2, 4);
    }
}

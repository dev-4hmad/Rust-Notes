# Rust Patterns & Matching (Ch. 19)

- **Patterns**: tuple, struct, enum, reference, slice, ranges, OR (`|`), `_`.
- **Where used**: `let`, `match`, `if let`, `while let`, function parameters.
- **Refutability**: 
  - **Refutable** patterns can fail to match (e.g., `Some(x)`).
  - **Irrefutable** patterns always match (e.g., `let x = 5`).
- **Matching**: `match` arms execute code when pattern matches; `_` catches all.

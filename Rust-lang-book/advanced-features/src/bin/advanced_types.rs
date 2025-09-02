//! Advanced Types: type aliases, the never type `!`, dynamically sized types, and coherence notes.

use std::fmt;

// 1) Type aliases
type Kilometers = i32;
type MyResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// 2) The never type `!`
// A function that never returns (diverges).
#[allow(dead_code)]
fn forever() -> ! {
    loop { /* do work, log, sleep, etc. */ }
}

// 3) Dynamically sized types and `?Sized`
fn print_debug<T: fmt::Debug + ?Sized>(value: &T) {
    println!("{:?}", value);
}

fn main() -> MyResult<()> {
    // Type aliases are not new types; they're just synonyms.
    let distance: Kilometers = 10;
    let dx: i32 = 5;
    let total: Kilometers = distance + dx;
    println!("total km = {}", total);

    // Never type is used by `panic!`, `loop {}`, and `return` in expression position.
    let maybe_number = if total > 100 { 1 } else { 2 }; // branches return same type

    println!("maybe_number = {}", maybe_number);

    // `?Sized`—you can take references to DSTs like `str` or trait objects.
    let s: &str = "hello DST";
    print_debug(s);
    let obj: &dyn fmt::Debug = &("tuple", 123);
    print_debug(obj);

    Ok(())
}

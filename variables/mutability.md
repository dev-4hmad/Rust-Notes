- Variables are immutable by default.
- Use mut keyword to make them mutable.
``` rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
This won't print any error because x is mutable.
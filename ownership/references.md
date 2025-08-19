## Mutable and Immutable reference

- Cannot modify borrowed value
- Can create mutable reference
- Cannot create two mutable references in the same scope or side by side like one after one
- Can't create mutable and immutable at the same time
- The concept of scopes works here
- Dangling references
- Creating a string slice

#### Immutable 

````rust
fn main() {
    let s = String::from("hello"); // This is a immutable reference which will throw error because we tried to change  the s, which is immutable

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
````
#### Mutable

````rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
````

#### Slice Reference
````rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
````

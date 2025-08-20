## Structs

````rust
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
````
To get a specific value from a struct, we use dot notation. For example, to access this user’s email address, we use user1.email

````rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // A short hand so we don't repeat username: username,
        email,
        sign_in_count: 1,
    }
}
````

#### Tuple Structs

- Having fields but no names

````rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
````

#### Unit Like Structs

- Don't have any fields

````rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
````

#### Impl and method syntax

These methods begin with impl, Often called as associated functions because they are associated with an impl
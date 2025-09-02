//! Advanced Traits: associated types, default type params (operator overloading),
//! fully qualified syntax, supertraits, and the newtype pattern.

use std::fmt;
use std::ops::Add;

// 1) Associated types
trait Contains {
    type Item;
    fn contains(&self, item: &Self::Item) -> bool;
}

impl<T: PartialEq> Contains for Vec<T> {
    type Item = T;
    fn contains(&self, item: &Self::Item) -> bool {
        self.iter().any(|x| x == item)
    }
}

// 2) Default type parameters & operator overloading
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// 3) Fully qualified syntax
trait Greet { fn greet(&self) -> String; }
trait Shout { fn greet(&self) -> String; }

struct Person(&'static str);
impl Greet for Person { fn greet(&self) -> String { format!("Hello, {}.", self.0) } }
impl Shout for Person { fn greet(&self) -> String { format!("HEY, {}!", self.0) } }

// 4) Supertraits: a trait that requires another trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let s = self.to_string();
        let len = s.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", s);
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug)]
struct Wrapper(i32);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wrapper({})", self.0)
    }
}
impl OutlinePrint for Wrapper {}

fn main() {
    // Associated types
    let v = vec![1, 2, 3];
    let has_two = <Vec<i32> as Contains>::contains(&v, &2);
    println!("Vec contains 2? {}", has_two);

    // Operator overloading with Add
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    println!("p1 + p2 = {:?}", p1 + p2);

    // Fully qualified syntax when methods collide
    let p = Person("Ada");
    println!("{}", Greet::greet(&p));
    println!("{}", <Person as Shout>::greet(&p));

    // Supertrait default method using Display
    Wrapper(42).outline_print();
}

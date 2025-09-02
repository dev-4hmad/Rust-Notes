//! Advanced Functions & Closures: function pointers, closure traits, and returning closures.

// 1) Function pointers
fn add_one(x: i32) -> i32 { x + 1 }

fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// 2) Closure trait bounds: FnOnce, FnMut, Fn
fn takes_fn_once<F: FnOnce(String)>(f: F) {
    f(String::from("consumed"));
}

fn takes_fn_mut<F: FnMut(i32) -> i32>(mut f: F) {
    let _ = f(1);
    let _ = f(2);
}

fn takes_fn<F: Fn(&str) -> usize>(f: F) {
    let len = f("hello");
    println!("len = {}", len);
}

// 3) Returning a closure with `impl Trait`
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn main() {
    // Function pointers
    let result = apply_twice(add_one, 3);
    println!("apply_twice(add_one, 3) = {}", result);

    // Closure trait differences
    let s = String::from("owned");
    takes_fn_once(|msg| {
        // moves `s` into the closure environment on first (and only) call
        println!("FnOnce: {} + {}", s, msg);
    });

    let mut total = 0;
    takes_fn_mut(|x| { total += x; total });

    takes_fn(|text| text.len());

    // Returning closures
    let add5 = make_adder(5);
    println!("add5(10) = {}", add5(10));
}

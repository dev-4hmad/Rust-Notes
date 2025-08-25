// A generic function that works with any type T
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![10, 20, 5, 40, 15];
    let chars = vec!['a', 'z', 'k', 'm'];

    println!("Largest number: {}", largest(&numbers));
    println!("Largest char: {}", largest(&chars));
}

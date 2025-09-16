fn between(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}
fn main() {
    println!("{:?}", between(1, 4)); // [1, 2, 3, 4]
}

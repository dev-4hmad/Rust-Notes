fn get_age(age: &str) -> Option<u32> {
    age.chars().next()?.to_digit(10)
}

fn main() {
    println!("{:?}", get_age("5 years old")); // Some(5)
    println!("{:?}", get_age(""));            // None
}

// A function that returns the longer of two string slices
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("short");
    let str2 = String::from("a much longer string");

    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is: {}", result);
}

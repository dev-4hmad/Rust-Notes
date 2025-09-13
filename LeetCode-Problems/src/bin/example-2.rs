use std::collections::HashMap;

fn roman_to_int(s: &str) -> i32 {
    // Step 1: Roman numerals map
    let mut values = HashMap::new();
    values.insert('I', 1);
    values.insert('V', 5);
    values.insert('X', 10);
    values.insert('L', 50);
    values.insert('C', 100);
    values.insert('D', 500);
    values.insert('M', 1000);

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;

    // Step 2: iterate through string
    for i in 0..chars.len() {
        let value = values[&chars[i]];
        // Step 3: check subtraction rule
        if i + 1 < chars.len() && value < values[&chars[i + 1]] {
            result -= value;
        } else {
            result += value;
        }
    }

    result
}

fn main() {
    let test1 = "III";
    let test2 = "LVIII";
    let test3 = "MCMXCIV";
    let test4 = "X";

    println!("{} -> {}", test1, roman_to_int(test1));   // 3
    println!("{} -> {}", test2, roman_to_int(test2));   // 58
    println!("{} -> {}", test3, roman_to_int(test3));   // 1994
    println!("{} -> {}", test4, roman_to_int(test4));   // 1994
}

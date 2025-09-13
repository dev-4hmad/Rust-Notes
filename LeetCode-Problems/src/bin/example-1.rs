fn main() {
    let test_cases = vec![121, -121, 10, 12321, 0];

    for &x in &test_cases {
        if is_palindrome(x) {
            println!("{} is a palindrome ✅", x);
        } else {
            println!("{} is NOT a palindrome ❌", x);
        }
    }
}

// Function to check if a number is a palindrome
fn is_palindrome(x: i32) -> bool {
    // Negative numbers are never palindrome
    if x < 0 {
        return false;
    }

    let mut original = x;   // store original number
    let mut reversed = 0;   // this will store reversed number

    while original != 0 {
        let digit = original % 10;          // get last digit
        reversed = reversed * 10 + digit;  // append digit to reversed number
        original /= 10;                     // remove last digit from original
    }

    // If reversed number equals original number, it's a palindrome
    reversed == x
}

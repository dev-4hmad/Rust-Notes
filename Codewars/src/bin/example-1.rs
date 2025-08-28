// ----------- Taking square of each each digit of a number and concatenating it


fn square_digits(num: u64) -> u64 {
    num.to_string() // convert number → string
        .chars() // iterate over characters
        .map(|c| c.to_digit(10).unwrap()) // turn each char into a digit
        .map(|d| d * d) // square each digit
        .map(|d| d.to_string()) // convert squared digit back to string
        .collect::<String>() // join all parts into one string
        .parse::<u64>() // parse back into u64
        .unwrap()
}

fn main() {
    println!("{}", square_digits(9119)); // 811181
    println!("{}", square_digits(765));  // 493625
}

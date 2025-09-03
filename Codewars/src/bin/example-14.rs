fn dig_pow(n: i64, p: i32) -> i64 {
    let digits: Vec<i64> = n.to_string()
                            .chars()
                            .map(|c| c.to_digit(10).unwrap() as i64)
                            .collect();

    let mut sum: i64 = 0;
    for (i, d) in digits.iter().enumerate() {
        sum += d.pow((p + i as i32) as u32);
    }

    if sum % n == 0 {
        sum / n
    } else {
        -1
    }
}

fn main() {
    println!("{}", dig_pow(89, 1));    // 1
    println!("{}", dig_pow(92, 1));    // -1
    println!("{}", dig_pow(695, 2));   // 2
    println!("{}", dig_pow(46288, 3)); // 51
}

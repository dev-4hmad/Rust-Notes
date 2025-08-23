fn main() {
    println!("--- Iterators ---");

    let nums = vec![1, 2, 3, 4, 5];

    // Consuming adaptor: sum
    let sum: i32 = nums.iter().sum();
    println!("Sum of nums = {}", sum);

    // Iterator adaptor: map
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled values: {:?}", doubled);

    // Filter example
    let evens: Vec<&i32> = nums.iter().filter(|x| *x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // Using closure with environment capture
    let threshold = 3;
    let greater: Vec<&i32> = nums.iter().filter(|&&x| x > threshold).collect();
    println!("Numbers greater than {}: {:?}", threshold, greater);

    // Iterators with enumerate
    for (index, value) in nums.iter().enumerate() {
        println!("Index {} has value {}", index, value);
    }

    println!("\n--- Custom Iterator ---");

    // Custom iterator by implementing Iterator trait
    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter::new();
    for num in counter {
        println!("Counter value: {}", num);
    }
}

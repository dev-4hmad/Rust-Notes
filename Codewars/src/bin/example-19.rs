// returning minimum and maximum from a list, arr
fn min_max(lst: &[i32]) -> (i32, i32) {
    let min = *lst.iter().min().unwrap();
    let max = *lst.iter().max().unwrap();
    (min, max)
}

fn main() {
    println!("{:?}", min_max(&[1, 2, 3, 4, 5]));       // (1, 5)
    println!("{:?}", min_max(&[2334454, 5]));          // (5, 2334454)
    println!("{:?}", min_max(&[1]));                   // (1, 1)
}

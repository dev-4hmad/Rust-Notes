fn reverse_letters(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic()) 
        .rev()                         
        .collect()                     
}
fn main() {
    println!("{}", reverse_letters("ahmad"));   
    println!("{}", reverse_letters("ultr53o?n")); 
}

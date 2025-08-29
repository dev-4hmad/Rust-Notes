// finding the vowels in a string
// use of .contains(), .filter(), and .count()



fn get_count(s: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    s.chars().filter(|c| vowels.contains(c)).count()
}
fn main(){
    println!("{}", get_count("ahmad"))
}
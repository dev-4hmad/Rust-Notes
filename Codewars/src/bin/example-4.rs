// Replace With Alphabet Position

fn alphabet_position(text: &str) -> String {

    text.chars().filter(|c|c.is_ascii_alphabetic()).map(|c| {
        let lower = c.to_ascii_lowercase();
        match lower {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
         _  => 0,
       }.to_string()
    }).collect::<Vec<_>>().join(" ")
}
fn main(){
    let s1 = "A wonderful day";
    println!("{}", alphabet_position(s1))
}
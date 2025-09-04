// Calculating probability
fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    ((blue_start - blue_pulled) as f32)/((blue_start - blue_pulled + red_start - red_pulled) as f32)
}
fn main() {
    println!("{}", guess_blue(5, 5, 2, 3));
}
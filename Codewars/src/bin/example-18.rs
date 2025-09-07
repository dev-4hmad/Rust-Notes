// finding third angle
fn other_angle(a: u32, b: u32) -> u32 {
    let sum = a+b;
    180 - sum
}
fn main(){
    println!("{}", other_angle(20, 15));
}
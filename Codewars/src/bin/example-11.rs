// making a number negative
fn make_negative(n: i32) -> i32 {
    if n > 0 {
        n*(-1)
    } else {
        n
    }
}
fn main(){
    let num = 10;
    println!("{}", make_negative(num))
}
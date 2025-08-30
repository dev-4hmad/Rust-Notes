// Convert the time to millisecond
fn past(h: i32, m: i32, s: i32) -> i32 {
    (h*3600000)+(m*60000)+(s*1000)
}
fn main(){
    let hour = 3;
    let minutes = 4;
    let sec = 2;
    println!("{}", past(hour, minutes, sec));
}
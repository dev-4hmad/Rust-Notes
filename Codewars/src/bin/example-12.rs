fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}
fn main(){
    let ss = "\nAhmad ";
    let cc = 10;
    println!("{}", repeat_str(ss, cc))
}
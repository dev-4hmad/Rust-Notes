fn remove_every_other<'a>(arr: &'a [&'a str]) -> Vec<&'a str> {
    arr.iter().step_by(2).copied().collect()
}
fn main() {
    let v = vec!["a", "b", "c"];
    println!("{:?}", remove_every_other(&v));
}
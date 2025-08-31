// .enumerate()

fn main() {
    let v = vec!["a", "b", "c"];
    v.iter().enumerate().for_each(|(i, val)| {
        println!("{}: {}", i, val);
    });
}
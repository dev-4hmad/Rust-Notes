// starts_with operator
fn are_you_playing_banjo(name: &str) -> String {
    if name.starts_with("R")|| name.starts_with("r"){
        format!("{name} plays banjo")
    } else {
        format!("{name} does not play banjo")
    }
}
fn main(){
    print!("{}", are_you_playing_banjo(&String::from("Roma")));
}
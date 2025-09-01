fn string_to_vec(st: &str)->Vec<&str>{
    st.split_whitespace().collect()
}
fn main(){
    let my_string = "I love coding";
    println!("{:?}", string_to_vec(my_string))
}
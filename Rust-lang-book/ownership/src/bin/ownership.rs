#[allow(unused_variables)]
fn main(){
    let s1 = String::from("hello");
    let s2 = s1; 

    // Results in an error, The ownership of s1 transferred to s2, use s1.clone()

    // println!("{s1}, world!");  

}
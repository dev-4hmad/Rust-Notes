// Convert temperatures between Fahrenheit and Celsius.
fn far_to_cel(f:f64)->f64{
    (f-32.0)*5.0/9.0
}
fn cel_to_far(c:f64)->f64{
    (c * 9.0/5.0) + 32.0
}
fn main(){
    let f:f64 = 10.0;
    let c:f64 = 12.0;

    // println!("{:?}",far_to_cel); Cannot call the function directly like this
    println!("{}°F -> {}°C",f, far_to_cel(f));
    println!("{}°C -> {}°F",c, cel_to_far(c));
}
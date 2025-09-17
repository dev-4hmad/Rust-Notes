// basicaly option type
fn sss(de:f64, n:f64)->Option<f64>{
    if de == 0.0{
        None
    } else {
        Some(de/n)
    }
}
fn main(){
    let div = (4324.0,43.0);
    println!("{:?}", sss(div.0, div.1))
}
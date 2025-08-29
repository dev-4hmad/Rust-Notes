// converting binary to decimal 

fn binary_slice_to_number(slice: &[u32]) -> u32 {
    // your code here
    slice.iter().fold(0, |acc, &bit|acc*2+bit)
}
fn main(){
    let x = [1,0,0,0,1];
    print!("{}", binary_slice_to_number(&x))
}
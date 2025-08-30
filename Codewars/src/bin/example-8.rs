// BMI calculator 

fn bmi(weight: u32, height: f32) -> &'static str {
    let bmi = weight as f32 / (height*height);
    if bmi <= 18.5{
        "Underweight"
    } else if bmi <= 25.0{
        "Normal"
    } else if bmi <= 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}
fn main(){
    println!("{}", bmi(60, 1.78)); // Normal
}
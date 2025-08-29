// Finding average

fn better_than_average(class_points: &[u16], your_points: u16) -> bool {
    let sum: u16 = class_points.iter().sum();
    let avg = sum as f32 / class_points.len() as f32;
    your_points as f32 > avg
}
fn main(){
    let my_class = [30, 49, 90, 79];
    let me = 99;
    println!("{}", better_than_average(&my_class, me))
}
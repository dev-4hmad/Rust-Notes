// Bring the garden module into scope for this crate.
pub mod garden;

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

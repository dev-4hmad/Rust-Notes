enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn action(&self) {
        match self {
            TrafficLight::Red => println!("Stop!"),
            TrafficLight::Yellow => println!("Get Ready!"),
            TrafficLight::Green => println!("Go!"),
        }
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    light.action();
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light time: {}", red.time());
    println!("Yellow light time: {}", yellow.time());
    println!("Green light time: {}", green.time());
}

pub trait TrafficLightTime {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLightTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 30,
        }
    }
}

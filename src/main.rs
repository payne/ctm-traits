
struct Sedan {
    color: String,
}

impl Sedan {
    fn new(color: String) -> Self { Self { color } }
}

impl LandCapable for Sedan {
    fn drive(&self) {
        println!("{} Sedan is driving!", self.color);
    }
}

struct SUV;
impl LandCapable for SUV {
    fn drive(&self) {
        println!("SUV is driving!");
    }
}

trait LandCapable {
    fn drive(&self);
}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn main() {
    let car = Sedan::new("Red".to_string());
    road_trip(&car);
    let suv = SUV;
    road_trip(&suv);
}

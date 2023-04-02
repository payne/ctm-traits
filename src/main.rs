
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
impl LandCapable for SUV {}

trait LandCapable {
    fn drive(&self) {
        println!("Driving! -- default implementation");
    }
}

// Note: dyn and impl are options here with performance tradeoffs
// ts 4:20 in the Code to the Moon video
// dyn uses a fat pointer and is slower -- two pointers:
// one to the data and one to the vtable
//
// impl uses a thin pointer and is faster -- one pointer
// impl -- is static dispatch
fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive();
}

fn main() {
    let car = Sedan::new("Red".to_string());
    road_trip(&car);
    let suv = SUV;
    road_trip(&suv);
}

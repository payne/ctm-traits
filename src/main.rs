struct Sedan {
    color: String,
}

impl Sedan {
    fn new(color: String) -> Self { Self { color } }

    fn drive(&self) {
        print!("{} Sedan is driving!", self.color);
    }
}


fn main() {
    let car = Sedan::new("Red".to_string());
    car.drive();
}

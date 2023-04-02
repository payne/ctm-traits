struct Sedan;
impl Sedan {
    fn drive(&self) {
        print!("Sedan is driving!");
    }
}


fn main() {
    let car = Sedan;
    car.drive();
}

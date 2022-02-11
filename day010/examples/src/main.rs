struct Shuttle<'a> {
    name: &'a str,
}

impl<'a> Shuttle<'a> {
    fn send_transmission(&self, msg: &str) -> &str {
        println!("Transmitting message: {}", msg);
        self.name
    }
}
fn main() {
    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0,
    };

    // we need to use a new string, otherwise the string from first vehicle would be moved
    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
}

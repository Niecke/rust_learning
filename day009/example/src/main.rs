struct Satelite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

/* Without default function
trait Description {
    fn describe(&self) -> String;
}
*/

// Without the default Satelite has to have its own implementation of the describe function
trait Description {
    fn describe(&self) -> String {
        String::from("an object fyling through space!")
    }
}

impl Description for Satelite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!(
            "the {} flying at {} miles high with {} crew members aboard!",
            self.name, self.altitude, self.crew_size
        )
    }
}

fn main() {
    let hubble = Satelite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}

fn main() {
    /* TRANSFORM OWNERSHIP */
    let mut rocket_fuel = String::from("RP-1");
    // here we shadow the first rocket_fuel
    let length = process_fuel(&mut rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);

    /* SLICE */
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word = &message[15..15 + 5];
    println!("last word is {}", last_word);

    let planets = [1, 2, 3, 4, 5, 6, 7, 8];
    let inner_planets: &[i32] = &planets[..4];
    println!("inner_plannets are {:?}", inner_planets);

    let first_word = get_first_word(&message);
    println!("first_word is {}", first_word);
}

fn process_fuel(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }

    &s
}

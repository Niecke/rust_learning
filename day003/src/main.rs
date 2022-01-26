fn main() {
    /* SHADOWING */
    let planet = "Earth";

    // this starts a block that is always executed; just build a new scope
    {
        println!("planet is {}", planet);
        // shadowed variables can be mut even though the other is not
        let mut planet = "Mars";
        println!("planet is {}", planet);
    }

    println!("planet is {}", planet);

    /* TRANSFORM OWNERSHIP */
    let rocket_fuel = String::from("RP-1");
    // here we shadow the first rocket_fuel
    let rocket_fuel = process_fuel(rocket_fuel);
    println!("rocket_fuel is {}", rocket_fuel);
}

fn process_fuel(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    new_fuel
}

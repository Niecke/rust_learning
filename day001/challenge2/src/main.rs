fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn celsius_to_fahrenheit(celsius_temp: f64) -> f64 {
    /*F = (1.8 x C + 32)*/

    /* SOLUTION */
    1.8 * celsius_temp + 32 as f64
}
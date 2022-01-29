use rand::prelude::*;
use std::io;

fn main() {
    let secret_number = thread_rng().gen_range(1..101);
    let mut guesses = 0;
    let mut number: i32;
    let mut buffer = String::new();

    loop {
        println!("Enter a numer: ");
        guesses += 1;
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input line.");
        number = buffer
            .trim()
            .parse::<i32>()
            .expect("Failed to parse the guess.");
        if number == secret_number {
            println!(
                "You have found the secret number of {}. It took {} guesses.",
                secret_number, guesses
            );
            break;
        } else if secret_number < number {
            println!(
                "The secret number is smaller than {} and this was your {} guess",
                number, guesses
            );
        } else if secret_number > number {
            println!(
                "The secret number is bigger than {} and this was your {} guess",
                number, guesses
            );
        }
        buffer = String::new();
    }
}

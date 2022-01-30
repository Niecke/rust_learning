use std::env;
use std::fs;

fn main() {
    if env::args().len() != 3 {
        eprintln!("Program  requires exactly 2 arguments! <file path> <search name>");
        std::process::exit(1);
    }

    let file_path = env::args().nth(1).unwrap();
    let name = env::args().nth(2).unwrap();
    println!("Searching for {} in {} ...", name, file_path);

    let contents = fs::read_to_string(file_path).unwrap();
    for (index, line) in contents.lines().enumerate() {
        if name == line {
            println!("Found {} on line {}.", name, index);
            return;
        }
    }
    println!("{} was not found in the file.", name);
}

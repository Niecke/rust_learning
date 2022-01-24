fn main() {
    let mut count = 0;

    // LOOP
    println!("** LOOP **");
    let loop_result = loop {
        println!("count: {}", count);
        if count >= 10 {
            break count;
        }
        count += 1;
    };

    println!("The result was: {}", loop_result * 10);
    count = 0;

    // WHILE
    println!("** WHILE **");
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    println!("The result was: {}", count);

    // FOR
    println!("** FOR **");
    let message = ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
    for item in message {
        print!("{}", item);
    }
    print!("\n");

    // FOR with iterator
    println!("** FOR iterator **");
    let message = ['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'r' {
            break;
        }
    }
    print!("\n");

    // MATRIX
    println!("** MATRIX **");
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for row in matrix {
        for num in row {
            print!("{}\t", num);
        }
        print!("\n");
    }
}

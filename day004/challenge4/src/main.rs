fn main() {
    let test1 = "We need more space.";
    assert_eq!(trimspaces(test1), "We need more space.");
    println!("Test 1 passed!");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trimspaces(&test2), "There's space in front.");
    println!("Test 2 passed!");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trimspaces(&test3[..]), "There's space to the rear.");
    println!("Test 3 passed!");

    let test4 = "  We're surrounded by space!   ";
    assert_eq!(trimspaces(test4), "We're surrounded by space!");
    println!("Test 4 passed!");

    let test5 = "    ";
    assert_eq!(trimspaces(test5), "");
    println!("Test 5 passed!");

    let test6 = "";
    assert_eq!(trimspaces(test6), "");
    println!("Test 6 passed!");

    let test7 = "🚀";
    assert_eq!(trimspaces(test7), "🚀");
    println!("Tests passed!");
}

fn trimspaces(s: &str) -> &str {
    // using two for loops would be much simpler :D
    let mut start: usize = 0;
    let mut end: usize = 0;

    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        // check if item is no space than it is the new end
        if item != b' ' {
            end = index + 1;
        }
        // aslong as end is unknown move start if item is empty
        if end == 0 && item == b' ' {
            start = index + 1;
        }
    }

    // catch case where there is nothing than spaces
    if end < start {
        return ""
    }
    
    &s[start..end]
}
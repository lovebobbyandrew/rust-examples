fn main() {
    // Constructor for a new, mutable string object.
    let mut choice = String::new();
    loop {
        println!("FOR, INFINITE, WHILE, OR EXIT?");
        // Empty choice of data from previous loop.
        choice.clear();
        // Crashes the program upon unreadable input.
        std::io::stdin()
            .read_line(&mut choice)
            .expect("read failure, panic invoked");
        // Allows for input case insensitivity.
        choice = choice.to_uppercase();
        if choice == "FOR\n" {
            let array = [100, 200, 300, 400, 500, 600, 700, 800, 900];
            println!("START FOR LOOP!");
            for element in array.iter() {
                println!("CURRENT ELEMENT IS: {}", element);
            }
            println!("END FOR LOOP!");
            continue;
        } else if choice == "INFINITE\n" {
            println!("START INFINITE LOOP!");
            loop {
                // Empty choice of data from previous loop.
                choice.clear();
                println!("AGAIN?\nYES OR NO?");
                // Crashes the program upon unreadable input.
                std::io::stdin()
                    .read_line(&mut choice)
                    .expect("read failure, panic invoked");
                // Allows for input case insensitivity.
                choice = choice.to_uppercase();

                if choice == "YES\n" {
                    println!("AGAIN!");
                    continue;
                } else if choice == "NO\n" {
                    choice.clear();
                    break;
                } else {
                    println!("INVALID USER INPUT.");
                    continue;
                }
            }
            println!("END INFINITE LOOP!");
            continue;
        } else if choice == "WHILE\n" {
            let mut count = 9;
            println!("START WHILE LOOP!");
            while count > 0 {
                println!("CURRENT VALUE IS: {}", count);
                count = count - 1;
            }
            println!("END WHILE LOOP!");
            continue;
        } else if choice == "EXIT\n" {
            println!("GOODBYE!");
            break;
        } else {
            println!("INVALID USER INPUT.");
            continue;
        }
    }
}

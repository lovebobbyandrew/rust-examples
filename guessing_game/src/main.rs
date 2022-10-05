// The gen_range method is implemented within the Rng trait.
use rand::Rng;

fn main() {
    // Generate a random number.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");
    loop {
        println!("Input your guess!");
        // The mut keyword renders variables mutable.
        let mut guess = String::new();
        // Crashes the program with message upon unreadable input errors.
        std::io::stdin()
            .read_line(&mut guess)
            .expect("read failure, panic invoked");
        // Handles invalid input errors.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only numerals are supported.");
                continue;
            },
        };
        println!("You guessed: {}", guess);
        // The Rust equivalent of a switch() statement.
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
}

// Bring flush() into scope for single line printing.
use std::io;
use std::io::Write;

fn main() {
    println!("Fibonacci Calculator");
    let mut n_string = String::new();
    let n_val;
    // Collect desired N value for Fibonacci sequence.
    loop {
        // Clear string from failed loop.
        n_string.clear();
        print!("N: ");
        // To allow for printing without a newline.
        io::stdout().flush().unwrap();
        // Crashes program upon unreadable input.
        std::io::stdin()
            .read_line(&mut n_string)
            .expect("read failure, panic invoked");
        match n_string.trim().parse::<u32>() {
            // If the string is a valid unsigned integer, then save as an integer.
            Ok(num) => {
                // 32-bit unsigned integers cannot hold the value of F(48) or higher.
                if num > 47 {
                    println!("(0 >= N <= 47)");
                    continue;
                }
                n_val = num;
                break;
            },
            Err(_) => {
                println!("(0 >= N <= 47)");
                continue;
            },
        };
    }
    let fib = fibonacci(n_val);
    println!("F({}) = {}", n_val, fib);
}

fn fibonacci(mut n: u32) -> u32 {
    let mut prev;
    let mut curr = 1;
    let mut next;
    // F(0) = 0
    if n == 0 {
        next = 0;
    } else {
        next = 1;
    }
    while n > 2 {
        prev = curr;
        curr = next;
        next = prev + curr;
        n -= 1;
    }
    next
}

// Allows for use of custom lib.rs file.
use minigrep::Config;

fn main() {
    // Will panic in case of invalid Unicode input.
    let args: Vec<String> = std::env::args().collect();
    // Returns error message and code upon incorrect number of arguments.
    // unwrap_or_else() allows for custom error handling.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln!() prints messages to standard error.
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    // if let is an alternative to unwrap_or_else() for error handling.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

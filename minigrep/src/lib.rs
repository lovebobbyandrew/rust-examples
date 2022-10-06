// Allows for the use of the Box trait object.
use std::error::Error;


// pub must be used to allow for main() to use Config struct objects.
pub struct Config {
    // pub must be used to allow for main() to use Config struct members.
    pub query: String,
    pub filename: String,
}

impl Config {
    // pub must be used to allow for main() to use Config struct methods.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Ensure the correct number of arguments.
        if args.len() < 3 {
            // Must explicitly return a string.
            return Err("not enough arguments");
        }
        // &args[0] is the executable.
        // clone() is not the most efficient solution, but simple.
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// Box is a trait object, so this function will return a type that implements the Error trait.
// Box allows for allocating values on the heap.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

// Searches the input text file for keyword, line by line.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // Allows for case insensitivity.
    let upper_query = query.to_uppercase();
    for line in contents.lines() {
        if line.to_uppercase().contains(&upper_query) {
            results.push(line);
        }
    }
    results
}

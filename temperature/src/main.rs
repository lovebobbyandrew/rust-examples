fn main() {
    let mut main_exit = false;
    while main_exit != true {
        // Default string object constructor.
        let mut input_unit = String::new();
        loop {
            input_unit.clear();
            println!("Input?");
            // Crashes program upon unreadable input.
            std::io::stdin()
                .read_line(&mut input_unit)
                .expect("read failure, panic invoked");
            // Allows for input case insensitivity.
            input_unit = input_unit.to_uppercase();
            // Remove newline character from end of string.
            input_unit.pop();
            match input_unit.as_str() {
                "C" => {
                    break;
                },
                "K" => {
                    break;
                },
                "F" => {
                    break;
                },
                // Default case is indicated by an underscore.
                _ => {
                    println!("(C, K, or F)");
                    continue;
                },
            }
        }
        let mut output_unit = String::new();
        loop {
            output_unit.clear();
            println!("Output?");
            std::io::stdin()
                .read_line(&mut output_unit)
                .expect("read failure, panic invoked");
            output_unit = output_unit.to_uppercase();
            output_unit.pop();
            match output_unit.as_str() {
                "C" => {
                    break;
                },
                "K" => {
                    break;
                },
                "F" => {
                    break;
                },
                _ => {
                    println!("(C, K, or F)");
                    continue;
                },
            }
        }
        let mut temp_string = String::new();
        let temp_val: f32;
        loop {
            temp_string.clear();
            println!("Temperature?");
            std::io::stdin()
                .read_line(&mut temp_string)
                .expect("read failure, panic invoked");
            // Ensures the user inputs a valid number.
            match temp_string.trim().parse::<f32>() {
                // If the string is a valid integer, then save as an integer.
                Ok(num) => {
                    temp_val = num;
                    break;
                },
                Err(_) => {
                    println!("(45, 83, 108, etc.)");
                    continue;
                },
            };
        }
        let output_val;
        // Convert the current unit to the desired unit.
        match input_unit.as_str() {
            "C" => {
                match output_unit.as_str() {
                    "C" => {
                        output_val = temp_val;
                    },
                    "K" => {
                        output_val = temp_val + 273.15;
                    },
                    "F" => {
                        output_val = temp_val * 1.8 + 32.0;
                    },
                    _ => {
                        // Prints a failure message, cleans the stack, and exits the program.
                        panic!("memory corrupted, panic invoked");
                    },
                };
            },
            "K" => {
                match output_unit.as_str() {
                    "C" => {
                        output_val = temp_val - 273.15;
                    },
                    "K" => {
                        output_val = temp_val;
                    },
                    "F" => {
                        output_val = 1.8 * (temp_val - 273.15) + 32.0;
                    },
                    _ => {
                        panic!("memory corrupted, panic invoked");
                    },
                };
            },
            "F" => {
                match output_unit.as_str() {
                    "C" => {
                        output_val = (temp_val - 32.0) * 0.5556;
                    },
                    "K" => {
                        output_val = ((temp_val - 32.0) * 5.0) / 9.0 + 273.15;
                    },
                    "F" => {
                        output_val = temp_val;
                    },
                    _ => {
                        panic!("memory corrupted, panic invoked");
                    },
                };
            },
            _ => {
                panic!("memory corrupted, panic invoked");
            },
        };
        println!("{} {} = {} {}", temp_val, input_unit, output_val, output_unit);
        let mut repeat = String::new();
        // Decides whether to run the program again or exit.
        loop {
        println!("Continue?");
            std::io::stdin()
                .read_line(&mut repeat)
                .expect("read failure, panic invoked");
            repeat = repeat.to_uppercase();
            repeat.pop();
            match repeat.as_str() {
                "N" => {
                    main_exit = true;
                    println!("Farewell!");
                    break;
                },
                "Y" => {
                    break;
                },
                _ => {
                    println!("(N or Y)");
                    continue;
                },
            }
        }
    }
}

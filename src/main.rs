// Imports
use std::io::{stdout, stdin, Write};
use std::process::{exit};

// Meta Commands
const EXIT:&str = "EXIT\n";

// Exit Constants
const NO_ERROR_EXIT:i32 = 0;

// Input Positions
const META_COMMAND_POS:usize = 0;
const KEY_POS:usize = 0;
const VALUE_POS:usize = 1;

/**
 * Always prints the provided string to the output by always flushing the output buffer.
 */
fn definite_print(string_to_print: &str, println: bool) {
    if println {
        println!("{}", string_to_print);
    } else {
        print!("{}", string_to_print);
    }

    stdout().flush().expect("flush failed");
}

/**
 * This method will interpret the input of the command and properly adjust totals of tally.
 */
fn interpret_input() {

}

/**
 * Gathers input from the CLI and places the resultant values into the string storage memory 
 * pointed two by the two pointers provided.
 */
fn gather_input(key: &mut String, value: &mut String) -> bool {
    definite_print(">: ", false);
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let split_input = input.split(|c| (c == ' ')).collect::<Vec<&str>>();

    // Potentially a valid meta command
    if split_input.len() == 1 {
        if EXIT.to_string().eq(split_input[META_COMMAND_POS]) {
            exit(NO_ERROR_EXIT);
        } else {
            println!("Thing {} {}", split_input[META_COMMAND_POS], EXIT.to_string());
            stdout().flush().expect("flush failed");
        }
    }

    // Actually gather values from the CLI
    if split_input.len() == 2 {
        *key = split_input[KEY_POS].to_string();
        *value = split_input[VALUE_POS].to_string();
        return true;
    }

    return false;
}

/**
 * The main method.
 */
fn main() {
    let mut key = String::new();
    let mut value = String::new();

    // Main Loop, will have to turn this into a separate class in the near future.
    loop {
        if gather_input(&mut key, &mut value) {
            println!("Provided Key {} : Value {}", key, value);
        }
    }
}

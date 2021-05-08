// Imports
use std::collections::HashMap;
use std::io::{stdout, stdin, Write};
use std::process::{exit};

// Meta Commands
const EXIT:&str = "EXIT\n";
const SAVE:&str = "SAVE\n";

// Exit Constants
const NO_ERROR_EXIT:i32 = 0;

// Input Positions
const META_COMMAND_POS:usize = 0;
const KEY_POS:usize = 0;
const VALUE_POS:usize = 1;

// Math Constants
const PLUS:&str = "+";
const MINUS:&str = "-";

// Our Table
item table::HashMap = HashMap::new();

/**
 * Always prints the provided string to the output by always flushing the output buffer.
 */
fn definite_print(string_to_print: String, println: bool) {
    if println {
        println!("{}", string_to_print);
    } else {
        print!("{}", string_to_print);
    }

    stdout().flush().expect("flush failed");
}

/**
 * Discerns the integer tally value provided. Any decimals will be ignored.
 */ 
fn evaluate_value(&value) -> i32 {
    if PLUS.to_string().eq(value.to_string().substring(0,1)) {
        return value.split(|c| c == PLUS).parse::<i32>;
    } else {
        return -1 * value.split(|c| c == MINUS).parse::<i32>;
    }
}

/**
 * This method will interpret the input of the command and properly adjust totals of tally.
 */
fn interpret_input(&key, &value) {
    if table.get(key) == None {
        table.insert(key, evaluate_value(value));
    } else {
        if let Some(value) = map.get_mut(key) {
            *value += evaluate_value(value);
        }
    }
}

/**
 * Loads the data into the table from a provided save file name.
 */
fn load_data() {

}

/**
 * Saves the data from the table into a provided save file name.
 */
fn save_data() {

}

/**
 * Gathers input from the CLI and places the resultant values into the string storage memory 
 * pointed two by the two pointers provided.
 */
fn gather_input(key: &mut String, value: &mut String) -> bool {
    definite_print(format!(">: "), false);
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let split_input = input.split(|c| (c == ' ')).collect::<Vec<&str>>();

    // Potentially a valid meta command
    if split_input.len() == 1 {
        if EXIT.to_string().eq(split_input[META_COMMAND_POS]) {
            exit(NO_ERROR_EXIT);
        } else if SAVE.to_string().eq(split_input[META_COMMAND_POS]) {
            save_data();
        } else {
            definite_print(
                format!("Thing {} {}", split_input[META_COMMAND_POS], EXIT.to_string()), true
            );
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
    // If file provided, load it into the class definied struct
    

    let mut key = String::new();
    let mut value = String::new();

    // Main Loop, will have to turn this into a separate class in the near future.
    loop {
        if gather_input(&mut key, &mut value) {
            definite_print(format!("Provided Key {} : Value {}", key, value), true);
            interpret_input(&key, &value);
        }
    }
}

// Imports
use std::collections::HashMap;
use std::io::{stdout, stdin, Write};
use std::process::{exit};

// Meta Commands
const EXIT:&str = "EXIT";
const SAVE:&str = "SAVE";

// Exit Constants
const NO_ERROR_EXIT:i32 = 0;

// Input Positions
const META_COMMAND_POS:usize = 0;
const KEY_POS:usize = 0;
const VALUE_POS:usize = 1;
const PARSED_VALUE_POS:usize = 1;

// Math Constants
const PLUS:char = '+';
const MINUS:char = '-';

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
fn evaluate_value(value: &str) -> i32 {
    if PLUS.to_string().eq(&value[..1]) {
        let cut_value = value.split(|c| (c == PLUS)).collect::<Vec<&str>>();
        definite_print(cut_value[PARSED_VALUE_POS].to_string(), true);
        return cut_value[PARSED_VALUE_POS].parse::<i32>().unwrap();
    } else if MINUS.to_string().eq(&value[..1]) {
        let cut_value = value.split(|c| (c == MINUS)).collect::<Vec<&str>>();
        return -1 * cut_value[PARSED_VALUE_POS].parse::<i32>().unwrap();        
    } else {
        return 0;
    }
}

/**
 * This method will interpret the input of the command and properly adjust totals of tally.
 */
fn interpret_input(key: &str, value: &str, table: &mut HashMap<String, i32>) {
    if !(*table).contains_key(&key.to_string()) {
        (*table).insert(key.to_string(), evaluate_value(value));
    } else {
        // Get rid of "Some"
        if let Some(key_value) = (*table).get_mut(key) {
            *key_value += evaluate_value(value);
        }
    }

    definite_print(format!("New Value: {}: {:?}", key, (*table).get(key)), true);
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

    let split_input = input.trim_end().split(|c| (c == ' ')).collect::<Vec<&str>>();

    // Potentially a valid meta command
    if split_input.len() == 1 {
        if EXIT.to_string().eq(split_input[META_COMMAND_POS]) {
            exit(NO_ERROR_EXIT);
        } else if SAVE.to_string().eq(split_input[META_COMMAND_POS]) {
            save_data();
        } else {
            definite_print(
                format!("Meta Command {} Not Comprehendable", split_input[META_COMMAND_POS]), true
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
    
    // Our Table
    let mut table:HashMap<String, i32> = HashMap::new();

    let mut key = String::new();
    let mut value = String::new();

    // Main Loop, will have to turn this into a separate class in the near future.
    loop {
        if gather_input(&mut key, &mut value) {
            definite_print(format!("Provided Key {} : Value {}", key, value), true);
            interpret_input(&key, &value, &mut table);
        }
    }
}

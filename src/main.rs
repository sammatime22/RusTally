// Imports
use std::collections::HashMap;
use std::env::{args};
use std::fs::{File};
use std::io::{BufRead, BufReader, stdout, stdin, Write};
use std::process::{exit};

// Meta Commands
const EXIT:&str = "EXIT";
const SAVE:&str = "SAVE";

// Exit Constants
const NO_ERROR_EXIT:i32 = 0;

// Input Positions
const FILENAME_PROVIDED:usize = 0;
const FILENAME_POS:usize = 1;
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
        if let Some(key_value) = (*table).get_mut(key) {
            *key_value += evaluate_value(value);
        }
    }

    definite_print(format!("New Value: {}: {:?}", key, (*table).get(key).unwrap()), true);
}

/**
 * Loads the data into the table from a provided save file name.
 */
fn load_data(filename: &str, table: &mut HashMap<String, i32>) {
    let file = File::open(filename.to_string()).unwrap();

    let file_reader = BufReader::new(file);

    for line in file_reader.lines() {
        let unwrapped_line = line.unwrap();
        let split_line = unwrapped_line.split(|c| (c == ':')).collect::<Vec<&str>>();

        if split_line.len() == 2 {
            interpret_input(split_line[KEY_POS], split_line[VALUE_POS], table);
        }
    }

    definite_print(format!("Finished loading {}", filename), false);
}

/**
 * Saves the data from the table into a provided save file name. This will overwrite the provided filename.
 */
fn save_data(filename: &str, table: &mut HashMap<String, i32>) {
    let mut file = File::create(filename);

    for (key, value) in table.iter() {
        let mut value_signage = "";
        if *value > 0 {
            value_signage = "+";
        }
        let line = format!("{}:{}{}", key, value_signage, value);
        writeln!(&mut file, "{}", line.to_string());
    }

    definite_print(format!("Finished writing {}", filename), false);
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
            // Ask for filename to save to 
            definite_print(format!("Please provide the filename, no extensions: "), false);
            let mut filename = String::new();
            stdin().read_line(&mut filename).unwrap();
            *key = SAVE.to_string();
            *value = filename;
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
    // Our Table
    let mut table:HashMap<String, i32> = HashMap::new();

    // If file provided, load it into the class definied struct
    let args: Vec<String> = args().collect();
    if args.len() == FILENAME_PROVIDED {
        let ref filename = *args[FILENAME_POS];
        load_data(filename, &mut table);
    }

    let mut key = String::new();
    let mut value = String::new();

    // Main Loop, will have to turn this into a separate class in the near future.
    loop {
        if gather_input(&mut key, &mut value) {
            if !(SAVE.to_string().eq(&key)) {
                definite_print(format!("Provided Key {} : Value {}", key, value), true);
                interpret_input(&key, &value, &mut table);
            } else {
                save_data(&value, &mut table);
            }
        }
    }
}

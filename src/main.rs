// Imports
use std::collections::HashMap;
use std::env::{args};
use std::fs::{File};
use std::io::{BufRead, BufReader, stdout, stdin, Write};
use std::process::{exit};

// File Extensions
const SYAML:&str = "syaml";

// Meta Commands
const EXIT:&str = "EXIT";
const SAVE:&str = "SAVE";

// Exit Constants
const NO_ERROR_EXIT:i32 = 0;

// Input Positions
const FILENAME_PROVIDED:usize = 2;
const FILENAME_POS:usize = 1;
const META_COMMAND_POS:usize = 0;
const KEY_POS:usize = 0;
const VALUE_POS:usize = 1;
const PARSED_VALUE_POS:usize = 1;

// Math Constants
const PLUS:char = '+';
const MINUS:char = '-';


/// Always prints the provided string to the output by always flushing the output buffer.
///
/// # Arguments
/// * `string_to_print` - A String that will be printed
/// * `println` - A boolean defining if we will print a newline at the end (if true)
fn definite_print(string_to_print: String, println: bool) {
    if println {
        println!("{}", string_to_print);
    } else {
        print!("{}", string_to_print);
    }

    // Forces the print to occur
    stdout().flush().expect("flush failed");
}


/// Discerns the integer tally value provided. Any decimals will be ignored.
///
/// # Arguments
/// * `value` - The value to be evaluated
///
/// # Output
/// A 32-bit integer
fn evaluate_value(value: &str) -> i32 {
    // Interpret positive integers
    if PLUS.to_string().eq(&value[..1]) {
        let cut_value = value.split(|c| (c == PLUS)).collect::<Vec<&str>>();
        return cut_value[PARSED_VALUE_POS].parse::<i32>().unwrap();
    } 
    
    // Interpret negative integers
    else if MINUS.to_string().eq(&value[..1]) {
        let cut_value = value.split(|c| (c == MINUS)).collect::<Vec<&str>>();
        return -1 * cut_value[PARSED_VALUE_POS].parse::<i32>().unwrap();        
    } 
    
    // We will return 0 if we cannot determine if the input was positive or negative
    else {
        return 0;
    }
}


/// This method will interpret the input of the command and properly adjust totals of tally.
///
/// # Arguments
/// * `key` - The key for the value
/// * `value` - The value to be inserted
/// * `table` - A pointer to the table where the key/value pair will be updated or inserted
fn interpret_input(key: &str, value: &str, table: &mut HashMap<String, i32>) {
    if !(*table).contains_key(&key.to_string()) {
        // Make a new key if nonexistant
        (*table).insert(key.to_string(), evaluate_value(value));
    } else {
        // Update a key if it exists
        if let Some(key_value) = (*table).get_mut(key) {
            *key_value += evaluate_value(value);
        }
    }

    definite_print(format!("New Value: {}: {:?}", key, (*table).get(key).unwrap()), true);
}


/// Loads the data into the table from a provided save file name.
///
/// # Arguments
/// * `filename` - The filename of the file to be loaded
/// * `table` - A pointer to the table, which will have the data from the file inserted
fn load_data(filename: &str, table: &mut HashMap<String, i32>) {
    let file = File::open(filename.to_string()).unwrap();

    let file_reader = BufReader::new(file);

    // For each line of the file, split the line at the delimeter, and interpret it's key/value pair
    for line in file_reader.lines() {
        let unwrapped_line = line.unwrap();
        let split_line = unwrapped_line.split(|c| (c == ':')).collect::<Vec<&str>>();

        if split_line.len() == 2 {
            interpret_input(split_line[KEY_POS], split_line[VALUE_POS], table);
        }
    }

    definite_print(format!("Finished loading {}", filename), true);
}


/// Saves the data from the table into a provided save file name. This will overwrite the provided filename.
///
/// # Arguments
/// * `filename` - The name of the file to where the data will be written to
/// * `table` - The table of the program where the file's contents will come from
fn save_data(filename: &str, table: &mut HashMap<String, i32>) {
    let mut file = match File::create(format!("{}.{}", filename, SYAML)) {
        Err(why) => panic!("couldn't create {}, {}", filename, why),
        Ok(file) => file,
    };

    // For each key/value pair in the table, write that into the file
    for (key, value) in table.iter() {
        let mut value_signage = "";
        // Positive numbers will require a + string be added to their value
        if *value > 0 {
            value_signage = "+";
        }
        let line = format!("{}:{}{}", key, value_signage, value);
        match file.write_all(line.as_bytes()) {
            Err(why) => panic!("Couldn't write {}, {}", line, why),
            Ok(_) => definite_print(format!("Wrote {}", line), true),
        };
    }

    definite_print(format!("Finished writing {}", filename), true);
}


/// Gathers input from the CLI and places the resultant values into the string storage memory 
/// pointed two by the two pointers provided.
///
/// # Arguments
/// * `key` - A pointer to the position in memory which will hold the key
/// * `value` - A pointer to the position in memory which will hold the value
///
/// # Output
/// A boolean representing if the gather_input method got values to which the main loop can use
/// to move forward.
fn gather_input(key: &mut String, value: &mut String) -> bool {
    definite_print(format!(">: "), false);
    
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // Take in the key/value pair delimited by a space
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
            *value = filename.to_string().trim_end().to_string();
            return true;
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


/// The main method.
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
            // Add input to the table
            if !(SAVE.to_string().eq(&key)) {
                definite_print(format!("Provided Key {} : Value {}", key, value), true);
                interpret_input(&key, &value, &mut table);
            } 
            // Save input
            else {
                definite_print("Saving...".to_string(), true);
                save_data(&value, &mut table);
            }
        }
    }
}

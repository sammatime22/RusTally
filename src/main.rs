use ferris_says::say;
use std::io::{stdout, stdin, BufWriter};

const WELCOME_TO_RUSTALLY:&str = "Welcome to RusTally!";
const WELCOME_TO_RUSTALLY_SIZE:usize = 20;

const KEY_POS:usize = 0;
const VALUE_POS:usize = 1;

/**
 * Gathers input from the CLI and places the resultant values into the string storage memory 
 * pointed two by the two pointers provided.
 */
fn gather_input(mut key: &str, mut value: &str) {
    println!(">: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Malformed entry.");

    let split_input = input.split(|c| (c == ' ')).collect::<Vec<&str>>();

    if split_input.len() == 2 {
        (key) = split_input[KEY_POS];
        (value) = split_input[VALUE_POS];
    }
}

/**
 * The main method.
 */
fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(WELCOME_TO_RUSTALLY.as_bytes(), WELCOME_TO_RUSTALLY_SIZE, &mut writer).unwrap();

    let mut key = String::new();
    let mut value = String::new();

    // Main Loop, will have to turn this into a separate class in the near future.
    loop {
        gather_input(&mut key, &mut value);
        println!("Provided Key {} : Value {}", key, value)
    }
}

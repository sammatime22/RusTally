use ferris_says::say;
use std::io::{stdout, stdin, BufWriter};

const WELCOME_TO_RUSTALLY:&str = "Welcome to RusTally!";
const WELCOME_TO_RUSTALLY_SIZE:usize = 20;

const KEY_POS = 0;
const VALUE_POS = 1;

fn gather_input(&key, &value) {
    println!(">: ");
    let mut input = String::new();
    input = io::stdin().lock().lines().next().unwrap().unwrap();

    let split_input = input.split(|c| (c == ' '));
    (*key) = split_input[KEY_POS];
    (*value) = split_input[VALUE_POS];
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
    while true {
        gather_input(&key, &value);
        println!("Provided Key {} : Value {}", key, value)
    }
}

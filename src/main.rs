use ferris_says::say;
use std::io::{stdout, BufWriter};

const WELCOME_TO_RUSTALLY:&str = "Welcome to RusTally!";
const WELCOME_TO_RUSTALLY_SIZE:usize = 20;

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(WELCOME_TO_RUSTALLY.as_bytes(), WELCOME_TO_RUSTALLY_SIZE, &mut writer).unwrap();
}

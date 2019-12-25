use std::fs::File;
use std::io::{BufReader, BufRead};

use intcode::{State, ConsoleIo};

fn main() {
    let input = File::open("res/input.txt").unwrap();
    let mut buffered = BufReader::new(input);
    let mut program_text = String::new();
    buffered.read_line(&mut program_text);

    let mut program_state: State<ConsoleIo> = State::from(&program_text[..]);
    program_state.run();
}

use std::fs::File;
use std::io::{BufReader, BufRead};

use intcode::State;

fn main() -> Result<(), std::io::Error> {
    let input = File::open("res/input.txt")?;
    let mut buffered = BufReader::new(input);
    let mut program_text = String::new();
    buffered.read_line(&mut program_text)?;

    let mut boost_program = State::from(&program_text[..]);

    boost_program.run();

    Ok(())
}

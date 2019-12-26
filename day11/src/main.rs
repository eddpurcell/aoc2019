mod map_io;

use std::fs::File;
use std::io::{BufReader, BufRead};

use intcode::State;

use crate::map_io::RobotState;

fn main() -> Result<(), std::io::Error> {
    let input = File::open("res/input.txt")?;
    let mut buffered = BufReader::new(input);
    let mut program_text = String::new();
    buffered.read_line(&mut program_text)?;

    let robot_state = RobotState::new();
    let mut paint_bot = State::new(&program_text[..], robot_state);
    paint_bot.run();
    println!("Part 1: {}", paint_bot.io.hull.len());

    let robot_state = RobotState::new_white_start();
    let mut paint_bot = State::new(&program_text[..], robot_state);
    paint_bot.run();
    println!("Part 2:\n");
    paint_bot.io.print_hull();
    Ok(())
}

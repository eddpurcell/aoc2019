use std::convert::{From, Into};
use std::collections::HashMap;

use intcode::{ReadInt, WriteInt, Err};

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Default for Color {
    fn default() -> Self { Color::Black }
}

impl From<i128> for Color {
    fn from(bin: i128) -> Self {
        match bin {
            0 => Color::Black,
            1 => Color::White,
            n => panic!("Invalid option: {}", n),
        }
    }
}

impl Into<i128> for Color {
    fn into(self) -> i128 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

impl Into<char> for Color {
    fn into(self) -> char {
        match self {
            Color::Black => '.',
            Color::White => '#',
        }
    }
}

enum Turn {
    Left90,
    Right90,
}

impl From<i128> for Turn {
    fn from(bin: i128) -> Self {
        match bin {
            0 => Turn::Left90,
            1 => Turn::Right90,
            n => panic!("Invalid option: {}", n),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn modify(self, turn: &Turn) -> Self {
        match turn {
            Turn::Left90 => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            Turn::Right90 => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        }
    }

    fn move_forward(&self, pos: (i128, i128)) -> (i128, i128) {
        match self {
            Direction::Up => (pos.0, pos.1 + 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 - 1),
            Direction::Left => (pos.0 - 1, pos.1),
        }
    }
}

#[derive(Clone, Copy)]
enum OutputAction {
    Paint,
    Turn,
}

impl OutputAction {
    fn next(self) -> Self {
        match self {
            OutputAction::Paint => OutputAction::Turn,
            OutputAction::Turn => OutputAction::Paint,
        }
    }
}

pub struct RobotState {
    pos: (i128, i128),
    dir: Direction,
    pub hull: HashMap<(i128, i128), Color>,
    next_action: OutputAction,
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            pos: (0, 0),
            dir: Direction::Up,
            hull: HashMap::new(),
            next_action: OutputAction::Paint,
        }
    }

    pub fn new_white_start() -> Self {
        let mut hull = HashMap::new();
        hull.insert((0, 0), Color::White);
        Self {
            pos: (0, 0),
            dir: Direction::Up,
            hull,
            next_action: OutputAction::Paint,
        }
    }

    // Upside down... will figure out later :\
    pub fn print_hull(&self) {
        let x_min = self.hull.keys().map(|p| p.0).min().unwrap() - 1;
        let x_max = self.hull.keys().map(|p| p.0).max().unwrap() + 2;
        let y_min = self.hull.keys().map(|p| p.1).min().unwrap() - 1;
        let y_max = self.hull.keys().map(|p| p.1).max().unwrap() + 2;

        for y in y_min..y_max {
            for x in x_min..x_max {
                match self.hull.get(&(x, y)) {
                    Some(&c) => {
                        let ch: char = Color::into(c);
                        print!("{}", ch);
                    },
                    None => {
                        let ch: char = Color::into(Color::Black);
                        print!("{}", ch);
                    }, 
                }
            }
            println!("");
        }
    }
}

impl ReadInt for RobotState {
    fn read(&mut self) -> Result<i128, Err> {
        Ok(Color::into(*self.hull.entry(self.pos.clone()).or_default()))
    }
}

impl WriteInt for RobotState {
    fn write(&mut self, n: i128) {
        match self.next_action {
            OutputAction::Paint => {
                self.hull.insert(self.pos, Color::from(n));
            },
            OutputAction::Turn => {
                self.dir = self.dir.modify(&Turn::from(n));
                self.pos = self.dir.move_forward(self.pos);
            },
        };
        self.next_action = self.next_action.next();
    }
}

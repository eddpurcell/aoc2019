use std::convert::{From};
use std::collections::HashMap;

use intcode::{State, ReadInt, WriteInt, Err};

fn main() {
    let program_text = include_str!("../res/input.txt");
    let mut state = State::new(program_text, GameIo::new());

    state.run();

    println!("Part 1: {}", state.io.tiles.values().filter(|&t| *t == Tile::Block).count());

    state.reset();
    state.set_mem(0, 2);
    state.run();
    println!("Part 2: {}", state.io.score);
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl From<i128> for Tile {
    fn from(n: i128) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            x => panic!("What input is this? {}", x),
        }
    }
}

#[derive(Clone, Copy)]
enum OutputAction {
    XPos,
    YPos,
    TileType,
}

impl OutputAction {
    fn next(self) -> Self {
        match self {
            Self::XPos => Self::YPos,
            Self::YPos => Self::TileType,
            Self::TileType => Self::XPos,
        }
    }
}

struct GameIo {
    tiles: HashMap<(i128, i128), Tile>,
    ball: (i128, i128),
    paddle: (i128, i128),
    output_action: OutputAction,
    last_x: i128,
    last_y: i128,
    score: i128,
}

impl GameIo {
    fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            ball: (-1, -1),
            paddle: (-1, -1),
            output_action: OutputAction::XPos,
            last_x: -1,
            last_y: -1,
            score: 0,
        }
    }
}

impl ReadInt for GameIo {
    fn read(&mut self) -> Result<i128, Err> {
        if self.ball.0 > self.paddle.0 {
            Ok(1)
        } else if self.ball.0 < self.paddle.0 {
            Ok(-1)
        } else {
            Ok(0)
        }
    }
}

impl WriteInt for GameIo {
    fn write(&mut self, n: i128) {
        match self.output_action {
            OutputAction::XPos => self.last_x = n,
            OutputAction::YPos => self.last_y = n,
            OutputAction::TileType => {
                if self.last_x == -1 && self.last_y == 0 {
                    self.score = n;
                } else {
                    let tile = Tile::from(n);
                    match tile {
                        Tile::Ball => self.ball = (self.last_x, self.last_y),
                        Tile::HPaddle => self.paddle = (self.last_x, self.last_y),
                        t => {
                            self.tiles.insert((self.last_x, self.last_y), t);
                        },
                    }
                }
            },
        };
        self.output_action = self.output_action.next();
    }
}

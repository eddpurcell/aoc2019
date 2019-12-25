use std::convert::From;

use crate::instruction::{Opcode, ParameterMode};
use crate::io::{ConsoleIo, ReadInt, WriteInt};

#[derive(Debug)]
pub struct State<T: ReadInt + WriteInt> {
    orig: Vec<i32>,
    program: Vec<i32>,
    pid: usize,
    pub io: T,
}

impl From<&str> for State<ConsoleIo> {
    fn from(program: &str) -> Self {
        State {
            orig: program.split(',').map(|n| n.trim_end().parse::<i32>().unwrap()).collect(),
            program: program.split(',').map(|n| n.trim_end().parse::<i32>().unwrap()).collect(),
            pid: 0,
            io: ConsoleIo::new(),
        }
    }
}

impl <T: ReadInt + WriteInt> State<T> {
    pub fn new(program: &str, io: T) -> State<T> {
        State {
            orig: program.split(',').map(|n| n.trim_end().parse::<i32>().unwrap()).collect(),
            program: program.split(',').map(|n| n.trim_end().parse::<i32>().unwrap()).collect(),
            pid: 0,
            io,
        }
    }

    pub fn run(&mut self) -> i32 {
        loop {
            let opcode = Opcode::from(self.program.get(self.pid).unwrap());
            match opcode {
                Opcode::Add(left_mode, right_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.program[self.pid + 3];
                    self.program[index as usize] = inputs.0 + inputs.1;
                    self.pid = self.pid + 4;
                },
                Opcode::Mul(left_mode, right_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.program[self.pid + 3];
                    self.program[index as usize] = inputs.0 * inputs.1;
                    self.pid += 4;
                },
                Opcode::Input => {
                    let input = self.io.read().unwrap();
                    let index = self.get_one_input(ParameterMode::Immediate);
                    self.program[index as usize] = input;
                    self.pid += 2;
                },
                Opcode::Output(mode) => {
                    self.io.write(self.get_one_input(mode));
                    self.pid += 2;
                },
                Opcode::JumpIfTrue(input_mode, pointer_mode) => {
                    let inputs = self.get_two_inputs(input_mode, pointer_mode);
                    self.pid = if inputs.0 == 0 {
                        self.pid + 3
                    } else {
                        inputs.1 as usize
                    }
                },
                Opcode::JumpIfFalse(input_mode, pointer_mode) => {
                    let inputs = self.get_two_inputs(input_mode, pointer_mode);
                    self.pid = if inputs.0 != 0 {
                        self.pid + 3
                    } else {
                        inputs.1 as usize
                    }
                },
                Opcode::LessThan(left_mode, right_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.program[self.pid + 3];
                    self.program[index as usize] = if inputs.0 < inputs.1 { 1 } else { 0 };
                    self.pid += 4;
                },
                Opcode::Equals(left_mode, right_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.program[self.pid + 3];
                    self.program[index as usize] = if inputs.0 == inputs.1 { 1 } else { 0 };
                    self.pid += 4;
                },
                Opcode::Exit => break,
            };
        }
        self.program[0]
    }

    pub fn set_noun_verb(&mut self, noun: i32, verb: i32) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn reset(&mut self) {
        self.program = self.orig.clone();
        self.pid = 0;
    }

    pub fn new_io(&mut self, io: T) {
        self.io = io;
    }

    fn get_one_input(&self, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Immediate => self.program[self.pid + 1],
            ParameterMode::Position => self.program[self.program[self.pid + 1] as usize],
        }
    }

    fn get_two_inputs(&self, left_mode: ParameterMode, right_mode: ParameterMode) -> (i32, i32) {
        (
            match left_mode {
                ParameterMode::Immediate => self.program[self.pid + 1],
                ParameterMode::Position => self.program[self.program[self.pid + 1] as usize],
            },
            match right_mode {
                ParameterMode::Immediate => self.program[self.pid + 2],
                ParameterMode::Position => self.program[self.program[self.pid + 2] as usize],
            }
        )
    }

}

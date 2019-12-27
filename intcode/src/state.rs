use std::convert::From;

use std::collections::HashMap;

use crate::instruction::{Opcode, ParameterMode};
use crate::io::{ConsoleIo, ReadInt, WriteInt};

#[derive(Debug)]
pub struct State<T: ReadInt + WriteInt> {
    orig: Vec<i128>,
    program: Vec<i128>,
    pid: usize,
    pub io: T,
    rel_base: i128,
    extra_mem: HashMap<usize, i128>,
}

impl From<&str> for State<ConsoleIo> {
    fn from(program: &str) -> Self {
        State {
            orig: program.split(',').map(|n| n.trim_end().parse::<i128>().unwrap()).collect(),
            program: program.split(',').map(|n| n.trim_end().parse::<i128>().unwrap()).collect(),
            pid: 0,
            io: ConsoleIo::new(),
            rel_base: 0,
            extra_mem: HashMap::new(),
        }
    }
}

impl <T: ReadInt + WriteInt> State<T> {
    pub fn new(program: &str, io: T) -> State<T> {
        State {
            orig: program.split(',').map(|n| n.trim_end().parse::<i128>().unwrap()).collect(),
            program: program.split(',').map(|n| n.trim_end().parse::<i128>().unwrap()).collect(),
            pid: 0,
            io,
            rel_base: 0,
            extra_mem: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> i128 {
        loop {
            let opcode = Opcode::from(self.program.get(self.pid).unwrap());
            match opcode {
                Opcode::Add(left_mode, right_mode, index_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.get_resultant_index(self.pid + 3, index_mode);
                    self.set_data(index as usize, inputs.0 + inputs.1);
                    self.pid = self.pid + 4;
                },
                Opcode::Mul(left_mode, right_mode, index_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.get_resultant_index(self.pid + 3, index_mode);
                    self.set_data(index as usize, inputs.0 * inputs.1);
                    self.pid += 4;
                },
                Opcode::Input(mode) => {
                    let input = self.io.read().unwrap();
                    let index = self.get_resultant_index(self.pid + 1, mode);
                    self.set_data(index as usize, input);
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
                Opcode::LessThan(left_mode, right_mode, index_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.get_resultant_index(self.pid + 3, index_mode);
                    self.set_data(index as usize, if inputs.0 < inputs.1 { 1 } else { 0 });
                    self.pid += 4;
                },
                Opcode::Equals(left_mode, right_mode, index_mode) => {
                    let inputs = self.get_two_inputs(left_mode, right_mode);
                    let index = self.get_resultant_index(self.pid + 3, index_mode);
                    self.set_data(index as usize, if inputs.0 == inputs.1 { 1 } else { 0 });
                    self.pid += 4;
                },
                Opcode::RelBaseOffset(mode) => {
                    self.rel_base += self.get_one_input(mode);
                    self.pid += 2;
                },
                Opcode::Exit => break,
            };
        }
        self.program[0]
    }

    pub fn set_noun_verb(&mut self, noun: i128, verb: i128) {
        self.program[1] = noun;
        self.program[2] = verb;
    }

    pub fn set_mem(&mut self, pos: usize, value: i128) {
        self.program[pos] = value;
    }

    pub fn reset(&mut self) {
        self.program = self.orig.clone();
        self.pid = 0;
    }

    pub fn new_io(&mut self, io: T) {
        self.io = io;
    }

    fn get_data(&self, pos: usize) -> i128 {
        match self.program.get(pos) {
            Some(&n) => n,
            None => match self.extra_mem.get(&pos) {
                Some(&n) => n,
                None => 0,
            }
        }
    }

    fn set_data(&mut self, pos: usize, data: i128) {
        if pos < self.program.len() {
            self.program[pos] = data;
        } else {
            self.extra_mem.insert(pos, data);
        }
    }

    fn get_one_input(&self, mode: ParameterMode) -> i128 {
        match mode {
            ParameterMode::Immediate => self.get_data(self.pid + 1),
            ParameterMode::Position => self.get_data(self.get_data(self.pid + 1) as usize),
            ParameterMode::Relative => self.get_data((self.rel_base + self.get_data(self.pid + 1)) as usize),
        }
    }

    fn get_two_inputs(&self, left_mode: ParameterMode, right_mode: ParameterMode) -> (i128, i128) {
        (
            match left_mode {
                ParameterMode::Immediate => self.get_data(self.pid + 1),
                ParameterMode::Position => self.get_data(self.get_data(self.pid + 1) as usize),
                ParameterMode::Relative => self.get_data((self.rel_base + self.get_data(self.pid + 1)) as usize),
            },
            match right_mode {
                ParameterMode::Immediate => self.get_data(self.pid + 2),
                ParameterMode::Position => self.get_data(self.get_data(self.pid + 2) as usize),
                ParameterMode::Relative => self.get_data((self.rel_base + self.get_data(self.pid + 2)) as usize),
            }
        )
    }

    fn get_resultant_index(&self, pos: usize, mode: ParameterMode) -> usize {
        match mode {
            ParameterMode::Immediate => panic!("Not a valid parameter mode!"),
            ParameterMode::Position => self.get_data(pos) as usize,
            ParameterMode::Relative => (self.rel_base + self.get_data(pos)) as usize,
        }
    }
}

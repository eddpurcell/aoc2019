use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let input = File::open("res/input.txt").unwrap();
    let mut buffered = BufReader::new(input);
    let mut program_text = String::new();
    buffered.read_line(&mut program_text);

    let mut orig_program: Vec<usize> = Vec::new();
    for number in program_text.split(",") {
        orig_program.push(number.trim().parse::<usize>().unwrap());
    }
    let mut error_program = orig_program.clone();
    error_program[1] = 12;
    error_program[2] = 2;

    run(&mut error_program);
    println!("Part 1: {}", error_program[0]);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut new_program = orig_program.clone();
            new_program[1] = noun;
            new_program[2] = verb;
            run(&mut new_program);
            if new_program[0] == 19690720 {
                println!("Part 2: {:02}{:02}", noun, verb);
                break 'outer;
            }
        }
    }
}

fn run(program: &mut Vec<usize>) {
    let mut i = 0;
    loop {
        match program[i] {
            1 => {
                let x = program[program[i+1]];
                let y = program[program[i+2]];
                let ind_result = program[i+3];
                program[ind_result] = x + y;
                i += 4;
            },
            2 => {
                let x = program[program[i+1]];
                let y = program[program[i+2]];
                let ind_result = program[i+3];
                program[ind_result] = x * y;
                i += 4;
            },
            99 => break,
            _ => panic!("What?"),
        }
    }
}
        
#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn run_simple_addition() {
        let mut program = vec![1usize,0,0,0,99];

        run(&mut program);

        assert_eq!(2, program[0]);
    }

    #[test]
    fn run_simple_mult() {
        let mut program = vec![2usize, 3, 0, 3, 99];

        run(&mut program);
        
        assert_eq!(6, program[3]);
    }

    #[test]
    fn run_slightly_more_complex() {
        let mut program = vec![1usize, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut program);
        assert_eq!(30, program[0]);
        assert_eq!(2, program[4]);
    }
}

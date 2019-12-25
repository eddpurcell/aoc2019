mod io;

use std::fs::File;
use std::io::{BufReader, BufRead};

use std::sync::mpsc;
use std::thread;

use permutohedron::Heap;

use intcode::State;

use crate::io::{IterIo, ConcurrentIo};

fn main() -> Result<(), std::io::Error> {
    let input = File::open("res/input.txt")?;
    let mut buffered = BufReader::new(input);
    let mut program_text = String::new();
    buffered.read_line(&mut program_text)?;

    let mut amp_program = State::new(&program_text[..], IterIo::from(vec![]));

    let mut base_seq = vec![0,1,2,3,4];
    let possible_seqs = Heap::new(&mut base_seq);

    let mut max_output = -1;

    for seq in possible_seqs {
        let mut last_output = 0;

        for phase in seq.iter() {
            let io = IterIo::from(vec![*phase, last_output]);
            amp_program.new_io(io);
            amp_program.reset();
            amp_program.run();
            last_output = *amp_program.io.output.last().unwrap();
        }

        if last_output > max_output {
            max_output = last_output;
        }
    }

    println!("Part 1: {}", max_output);

    let mut base_seq = vec![5,6,7,8,9];
    let possible_seqs = Heap::new(&mut base_seq);
    let mut max_output = -1;

    for seq in possible_seqs {

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let (tx3, rx3) = mpsc::channel();
        let (tx4, rx4) = mpsc::channel();
        let (tx5, rx5) = mpsc::channel();

        let con_io1 = ConcurrentIo::new(rx1, mpsc::Sender::clone(&tx2));
        let con_io2 = ConcurrentIo::new(rx2, mpsc::Sender::clone(&tx3));
        let con_io3 = ConcurrentIo::new(rx3, mpsc::Sender::clone(&tx4));
        let con_io4 = ConcurrentIo::new(rx4, mpsc::Sender::clone(&tx5));
        let con_io5 = ConcurrentIo::new(rx5, mpsc::Sender::clone(&tx1));

        let mut amp1 = State::new(&program_text[..], con_io1);
        let mut amp2 = State::new(&program_text[..], con_io2);
        let mut amp3 = State::new(&program_text[..], con_io3);
        let mut amp4 = State::new(&program_text[..], con_io4);
        let mut amp5 = State::new(&program_text[..], con_io5);

        tx1.send(seq[0]).unwrap();
        tx2.send(seq[1]).unwrap();
        tx3.send(seq[2]).unwrap();
        tx4.send(seq[3]).unwrap();
        tx5.send(seq[4]).unwrap();
        tx1.send(0).unwrap();

        let thread1 = thread::spawn(move || {
            amp1.run();
        });
        let thread2 = thread::spawn(move || {
            amp2.run();
        });
        let thread3 = thread::spawn(move || {
            amp3.run();
        });
        let thread4 = thread::spawn(move || {
            amp4.run();
        });
        let thread5 = thread::spawn(move || {
            amp5.run();
            amp5.io.last_output.unwrap()
        });

        thread1.join().unwrap();
        thread2.join().unwrap();
        thread3.join().unwrap();
        thread4.join().unwrap();
        let output = thread5.join().unwrap();
        if output > max_output {
            max_output = output;
        }
    }
    println!("Part 2: {}", max_output);
    Ok(())
}

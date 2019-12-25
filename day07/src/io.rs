use std::convert::From;

use std::sync::mpsc::{Sender, Receiver};

use intcode::{ReadInt, WriteInt, Err};

pub struct IterIo {
    input: Box<dyn Iterator<Item = i32>>,
    pub output: Vec<i32>,
}

impl From<Vec<i32>> for IterIo {
    fn from(inputs: Vec<i32>) -> IterIo {
        IterIo {
            input: Box::new(inputs.into_iter()),
            output: Vec::new(),
        }
    }
}

impl ReadInt for IterIo {
    fn read(&mut self) -> Result<i32, Err> {
        match self.input.next() {
            Some(n) => Ok(n),
            None => Err(Err::NaN),
        }
    }
}

impl WriteInt for IterIo {
    fn write(&mut self, n: i32) {
        self.output.push(n);
    }
}

pub struct ConcurrentIo {
    input: Receiver<i32>,
    output: Sender<i32>,
    pub last_output: Option<i32>,
}

impl ConcurrentIo {
    pub fn new(rx: Receiver<i32>, tx: Sender<i32>) -> Self {
        ConcurrentIo {
            input: rx,
            output: tx,
            last_output: None,
        }
    }
}

impl ReadInt for ConcurrentIo {
    fn read(&mut self) -> Result<i32, Err> {
        match self.input.recv() {
            Ok(n) => {
                Ok(n)
            },
            Err(e) => {
                println!("{:?}", e);
                Err(Err::Waiting(e))
            },
        }
    }
}

impl WriteInt for ConcurrentIo {
    fn write(&mut self, n: i32) {
        match self.output.send(n) {
            Ok(_) => (),
            Err(_) => self.last_output = Some(n),
        };
    }
}

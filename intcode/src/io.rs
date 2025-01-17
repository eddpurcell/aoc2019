use rustyline::Editor;

#[derive(Debug)]
pub struct ConsoleIo {
    rl: Editor<()>
}

impl ConsoleIo {
    pub fn new() -> ConsoleIo {
        ConsoleIo {
            rl: Editor::<()>::new(),
        }
    }
}

pub trait ReadInt {
    fn read(&mut self) -> Result<i128, Err>;
}

#[derive(Debug)]
pub enum Err {
    NaN,
    Waiting(std::sync::mpsc::RecvError),
}

impl ReadInt for ConsoleIo {
    fn read(&mut self) -> Result<i128, Err> {
        match self.rl.readline("> ").unwrap().trim_end().parse() {
            Ok(n) => Ok(n),
            Err(_) => Err(Err::NaN),
        }
    }
}

pub trait WriteInt {
    fn write(&mut self, n: i128);
}

impl WriteInt for ConsoleIo {
    fn write(&mut self, n: i128) {
        println!("{}", n);
    }
}

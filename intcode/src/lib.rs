mod instruction;
mod state;
mod io;

pub use state::State;
pub use io::{ConsoleIo, ReadInt, WriteInt, Err};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

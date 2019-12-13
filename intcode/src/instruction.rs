use std::convert::From;

#[derive(PartialEq, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
}

#[derive(PartialEq, Debug)]
pub enum Opcode {
    Exit,
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
}

impl From<&i32> for Opcode {
    fn from(operation: &i32) -> Opcode {
        match &operation {
            1 => Opcode::Add(ParameterMode::Position, ParameterMode::Position),
            101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Position),
            1001 => Opcode::Add(ParameterMode::Position, ParameterMode::Immediate),
            1101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Immediate),
            2 => Opcode::Mul(ParameterMode::Position, ParameterMode::Position),
            102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Position),
            1002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Immediate),
            1102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Immediate),
            3 => Opcode::Input,
            4 => Opcode::Output(ParameterMode::Position),
            104 => Opcode::Output(ParameterMode::Immediate),
            5 => Opcode::JumpIfTrue(ParameterMode::Position, ParameterMode::Position),
            105 => Opcode::JumpIfTrue(ParameterMode::Immediate, ParameterMode::Position),
            1005 => Opcode::JumpIfTrue(ParameterMode::Position, ParameterMode::Immediate),
            1105 => Opcode::JumpIfTrue(ParameterMode::Immediate, ParameterMode::Immediate),
            6 => Opcode::JumpIfFalse(ParameterMode::Position, ParameterMode::Position),
            106 => Opcode::JumpIfFalse(ParameterMode::Immediate, ParameterMode::Position),
            1006 => Opcode::JumpIfFalse(ParameterMode::Position, ParameterMode::Immediate),
            1106 => Opcode::JumpIfFalse(ParameterMode::Immediate, ParameterMode::Immediate),
            7 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Position),
            107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Position),
            1007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Immediate),
            1107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Immediate),
            8 => Opcode::Equals(ParameterMode::Position, ParameterMode::Position),
            108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Position),
            1008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Immediate),
            1108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Immediate),
            99 => Opcode::Exit,
            n => panic!("`{}` is not a valid opcode!", n),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_correct_opcode() {
        assert_eq!(Opcode::Mul(ParameterMode::Immediate, ParameterMode::Position), Opcode::from(&1002));
    }
}

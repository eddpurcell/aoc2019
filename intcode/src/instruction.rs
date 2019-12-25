use std::convert::From;

#[derive(PartialEq, Debug)]
pub enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(PartialEq, Debug)]
pub enum Opcode {
    Exit,
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    RelBaseOffset(ParameterMode),
}

impl From<&i128> for Opcode {
    fn from(operation: &i128) -> Opcode {
        match &operation {
            1 => Opcode::Add(ParameterMode::Position, ParameterMode::Position, ParameterMode::Position),
            101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Position),
            1001 => Opcode::Add(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position),
            1101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Position),
            201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Position),
            1201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Position),
            2001 => Opcode::Add(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Position),
            2101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Position),
            2201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Position),
            20001 => Opcode::Add(ParameterMode::Position, ParameterMode::Position, ParameterMode::Relative),
            20101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Relative),
            21001 => Opcode::Add(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Relative),
            21101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Relative),
            20201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Relative),
            21201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Relative),
            22001 => Opcode::Add(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Relative),
            22101 => Opcode::Add(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Relative),
            22201 => Opcode::Add(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Relative),

            2 => Opcode::Mul(ParameterMode::Position, ParameterMode::Position, ParameterMode::Position),
            102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Position),
            1002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position),
            1102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Position),
            202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Position),
            1202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Position),
            2002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Position),
            2102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Position),
            2202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Position),
            20002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Position, ParameterMode::Relative),
            20102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Relative),
            21002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Relative),
            21102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Relative),
            20202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Relative),
            21202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Relative),
            22002 => Opcode::Mul(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Relative),
            22102 => Opcode::Mul(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Relative),
            22202 => Opcode::Mul(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Relative),

            3 => Opcode::Input(ParameterMode::Position),
            203 => Opcode::Input(ParameterMode::Relative),

            4 => Opcode::Output(ParameterMode::Position),
            104 => Opcode::Output(ParameterMode::Immediate),
            204 => Opcode::Output(ParameterMode::Relative),

            5 => Opcode::JumpIfTrue(ParameterMode::Position, ParameterMode::Position),
            105 => Opcode::JumpIfTrue(ParameterMode::Immediate, ParameterMode::Position),
            1005 => Opcode::JumpIfTrue(ParameterMode::Position, ParameterMode::Immediate),
            1105 => Opcode::JumpIfTrue(ParameterMode::Immediate, ParameterMode::Immediate),
            205 => Opcode::JumpIfTrue(ParameterMode::Relative, ParameterMode::Position),
            1205 => Opcode::JumpIfTrue(ParameterMode::Relative, ParameterMode::Immediate),
            2005 => Opcode::JumpIfTrue(ParameterMode::Position, ParameterMode::Relative),
            2105 => Opcode::JumpIfTrue(ParameterMode::Immediate, ParameterMode::Relative),
            2205 => Opcode::JumpIfTrue(ParameterMode::Relative, ParameterMode::Relative),

            6 => Opcode::JumpIfFalse(ParameterMode::Position, ParameterMode::Position),
            106 => Opcode::JumpIfFalse(ParameterMode::Immediate, ParameterMode::Position),
            1006 => Opcode::JumpIfFalse(ParameterMode::Position, ParameterMode::Immediate),
            1106 => Opcode::JumpIfFalse(ParameterMode::Immediate, ParameterMode::Immediate),
            206 => Opcode::JumpIfFalse(ParameterMode::Relative, ParameterMode::Position),
            1206 => Opcode::JumpIfFalse(ParameterMode::Relative, ParameterMode::Immediate),
            2006 => Opcode::JumpIfFalse(ParameterMode::Position, ParameterMode::Relative),
            2106 => Opcode::JumpIfFalse(ParameterMode::Immediate, ParameterMode::Relative),
            2206 => Opcode::JumpIfFalse(ParameterMode::Relative, ParameterMode::Relative),

            7 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Position, ParameterMode::Position),
            107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Position),
            1007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position),
            1107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Position),
            207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Position),
            1207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Position),
            2007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Position),
            2107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Position),
            2207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Position),
            20007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Position, ParameterMode::Relative),
            20107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Relative),
            21007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Relative),
            21107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Relative),
            20207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Relative),
            21207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Relative),
            22007 => Opcode::LessThan(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Relative),
            22107 => Opcode::LessThan(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Relative),
            22207 => Opcode::LessThan(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Relative),

            8 => Opcode::Equals(ParameterMode::Position, ParameterMode::Position, ParameterMode::Position),
            108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Position),
            1008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position),
            1108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Position),
            208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Position),
            1208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Position),
            2008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Position),
            2108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Position),
            2208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Position),
            20008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Position, ParameterMode::Relative),
            20108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Position, ParameterMode::Relative),
            21008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Relative),
            21108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Immediate, ParameterMode::Relative),
            20208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Position, ParameterMode::Relative),
            21208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Immediate, ParameterMode::Relative),
            22008 => Opcode::Equals(ParameterMode::Position, ParameterMode::Relative, ParameterMode::Relative),
            22108 => Opcode::Equals(ParameterMode::Immediate, ParameterMode::Relative, ParameterMode::Relative),
            22208 => Opcode::Equals(ParameterMode::Relative, ParameterMode::Relative, ParameterMode::Relative),

            9 => Opcode::RelBaseOffset(ParameterMode::Position),
            109 => Opcode::RelBaseOffset(ParameterMode::Immediate),
            209 => Opcode::RelBaseOffset(ParameterMode::Relative),

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

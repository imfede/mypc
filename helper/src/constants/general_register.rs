use super::{control_line::ControlLine, control_word::ControlWord};

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum GeneralRegister {
    A,
    B,
    C,
    D,
}

impl GeneralRegister {
    pub fn register_in(&self) -> ControlWord {
        match self {
            GeneralRegister::A => ControlWord::from_lines(&[ControlLine::RIE]),
            GeneralRegister::B => ControlWord::from_lines(&[ControlLine::RIE, ControlLine::RIL]),
            GeneralRegister::C => ControlWord::from_lines(&[ControlLine::RIE, ControlLine::RIH]),
            GeneralRegister::D => {
                ControlWord::from_lines(&[ControlLine::RIE, ControlLine::RIH, ControlLine::RIL])
            }
        }
    }

    pub fn register_out(&self) -> ControlWord {
        match self {
            GeneralRegister::A => ControlWord::from_lines(&[ControlLine::ROE]),
            GeneralRegister::B => ControlWord::from_lines(&[ControlLine::ROE, ControlLine::ROL]),
            GeneralRegister::C => ControlWord::from_lines(&[ControlLine::ROE, ControlLine::ROH]),
            GeneralRegister::D => {
                ControlWord::from_lines(&[ControlLine::ROE, ControlLine::ROH, ControlLine::ROL])
            }
        }
    }
}

impl From<u8> for GeneralRegister {
    fn from(value: u8) -> Self {
        match value {
            0 => GeneralRegister::A,
            1 => GeneralRegister::B,
            2 => GeneralRegister::C,
            3 => GeneralRegister::D,
            _ => panic!("Invalid general register"),
        }
    }
}

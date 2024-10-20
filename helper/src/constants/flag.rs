#[allow(clippy::upper_case_acronyms)]
pub enum Flag {
    /// Flag Zero, 1 if the output of the ALU is 0x00, 0 otherwise.
    FZ,
    /// Carry Out, 1 if the output of the ALU has the carry bit set, 0 otherwise.
    CO,
    /// A2 > A1, 1 if the contents of the ALU Register 1 is greater than the contents of ALU register 2, 0 otherwise.
    A2G1,
    /// Negative, 1 if the output of the ALU is less than 0, 0 otherwise.
    NEG,
}

impl Flag {
    fn value(&self) -> u8 {
        match self {
            Flag::FZ => 0b0001,
            Flag::CO => 0b0010,
            Flag::A2G1 => 0b0100,
            Flag::NEG => 0b1000,
        }
    }
}

// impl From<u8> for Flag {
//     fn from(byte: u8) -> Self {
//         match byte {
//             0b0001 => Flag::FZ,
//             0b0010 => Flag::CO,
//             0b0100 => Flag::A2G1,
//             0b1000 => Flag::NEG,
//             _ => unreachable!("Invalid flag"),
//         }
//     }
// }

pub struct Flags(u8);

impl Flags {
    pub fn from_flags(flags: &[Flag]) -> Self {
        let value = flags
            .iter()
            .map(|flag| flag.value())
            .fold(0, |acc, val| acc | val);
        Self(value)
    }
}

impl From<Vec<Flag>> for Flags {
    fn from(flags: Vec<Flag>) -> Self {
        Self::from_flags(&flags)
    }
}

impl<const N: usize> From<[Flag; N]> for Flags {
    fn from(flags: [Flag; N]) -> Self {
        Self::from_flags(&flags)
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        if value > 0b1111 {
            panic!("Flag out of bounds");
        }

        Flags(value)
    }
}

impl Flags {
    pub fn has(&self, flag: Flag) -> bool {
        (self.0 & flag.value()) != 0
    }
}
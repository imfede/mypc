use crate::constants::general_register::GeneralRegister;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug)]
pub enum AssemblyRegister {
    A,
    B,
    C,
    D,
}

impl From<AssemblyRegister> for GeneralRegister {
    fn from(src: AssemblyRegister) -> Self {
        match src {
            AssemblyRegister::A => GeneralRegister::A,
            AssemblyRegister::B => GeneralRegister::B,
            AssemblyRegister::C => GeneralRegister::C,
            AssemblyRegister::D => GeneralRegister::D,
        }
    }
}

impl AssemblyRegister {
    pub fn parse(input: &str) -> IResult<&str, AssemblyRegister> {
        map(one_of("ABCD"), |letter| match letter {
            'A' => AssemblyRegister::A,
            'B' => AssemblyRegister::B,
            'C' => AssemblyRegister::C,
            'D' => AssemblyRegister::D,
            _ => unreachable!("Invalid letter"),
        })(input)
    }
}
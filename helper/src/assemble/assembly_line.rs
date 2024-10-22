use crate::assemble::assembly_instruction::AssemblyInstruction;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug)]
pub enum Label {
    Absolute(String),
    Relative(String),
}

#[derive(Debug)]
pub struct AssemblyLine {
    label: Option<Label>,
    instruction: AssemblyInstruction,
}

impl AssemblyLine {
    pub fn parse(input: &str) -> IResult<&str, AssemblyLine> {
        map(
            tuple((
                opt(alt((AssemblyLine::parse_absolute_label, AssemblyLine::parse_relative_label))),
                multispace0,
                AssemblyInstruction::parse
            )),
            |(label, _, instruction)| AssemblyLine {
                label,
                instruction,
            },
        )(input)
    }

    fn parse_absolute_label(input: &str) -> IResult<&str, Label> {
        map(
            delimited(tag(":"), alphanumeric1, multispace1),
            |name: &str| Label::Absolute(name.to_string()),
        )(input)
    }

    fn parse_relative_label(input: &str) -> IResult<&str, Label> {
        map(
            delimited(tag("."), alphanumeric1, multispace1),
            |name: &str| Label::Relative(name.to_string()),
        )(input)
    }
}
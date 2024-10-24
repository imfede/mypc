use crate::assemble::assembly_instruction::AssemblyInstruction;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::combinator::{eof, map, opt};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Label {
    Absolute(String),
    Relative(String),
}

#[derive(Debug)]
pub struct AssemblyLine {
    pub label: Option<Label>,
    pub instruction: AssemblyInstruction,
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

pub fn parse_instructions(input: &str) -> IResult<&str, Vec<AssemblyLine>> {
    delimited(
        many0(alt((parse_comment, multispace1))),
        separated_list0(
            many1(alt((multispace1, parse_comment))),
            AssemblyLine::parse,
        ),
        eof,
    )(input)
}

fn parse_comment(input: &str) -> IResult<&str, &str> {
    map(tuple((
        tag("#"),
        is_not("\n"),
    )), |(_, comment)| comment)(input)
}
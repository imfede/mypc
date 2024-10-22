use crate::assemble::assembly_register::AssemblyRegister;
use crate::hex_u8::{hex_u16, hex_u8};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0, multispace1};
use nom::combinator::{map, not};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug)]
enum AbsoluteAddress {
    Label { name: String },
    HardCoded { address: u16 },
}

impl AbsoluteAddress {
    pub fn parse_label_name(input: &str) -> IResult<&str, String> {
        map(
            delimited(tag(":"), alphanumeric1, not(alphanumeric1)),
            |name: &str| name.to_string(),
        )(input)
    }
}

impl AbsoluteAddress {
    fn parse(input: &str) -> IResult<&str, AbsoluteAddress> {
        alt((
            map(
                AbsoluteAddress::parse_label_name,
                |name| AbsoluteAddress::Label { name },
            ),
            map(
                hex_u16,
                |value| AbsoluteAddress::HardCoded { address: value },
            ),
        ))(input)
    }
}

#[derive(Debug)]
enum Offset {
    Label { name: String },
    HardCoded { offset: i8 },
}

impl Offset {
    fn parse(input: &str) -> IResult<&str, Offset> {
        alt((
            map(
                delimited(tag("."), alphanumeric1, not(alphanumeric1)),
                |name: &str| Offset::Label { name: name.to_string() },
            ),
            map(
                nom::character::complete::i8,
                |value| Offset::HardCoded { offset: value },
            ),
        ))(input)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum AssemblyInstruction {
    NOP,
    MV { dst: AssemblyRegister, src: AssemblyRegister },
    MEMR { dst: AssemblyRegister },
    MEMW { src: AssemblyRegister },
    MSRL { src: AssemblyRegister },
    MSRH { src: AssemblyRegister },
    LI { dst: AssemblyRegister, value: u8 },
    ZERO { dst: AssemblyRegister },
    RTWL,
    RTWH,
    RTRL,
    RTRH,
    ADD { acc: AssemblyRegister, val: AssemblyRegister },
    SUB { acc: AssemblyRegister, val: AssemblyRegister },
    NAND { acc: AssemblyRegister, val: AssemblyRegister },
    XOR { acc: AssemblyRegister, val: AssemblyRegister },
    ADDI { dst: AssemblyRegister, value: u8 },
    INC { dst: AssemblyRegister },
    DEC { dst: AssemblyRegister },
    NEG { dst: AssemblyRegister },
    PJMP { address: AbsoluteAddress },
    JMP,
    JAL,
    RET,
    JCR { offset: Offset },
    JZR { offset: Offset },
    JNR { offset: Offset },
    JLTR { offset: Offset },
    SPSL { src: AssemblyRegister },
    SPSH { src: AssemblyRegister },
    PUSH,
    PULL,
    PEEK,
    SPOF { offset: u8 },
    HLT,
}

impl AssemblyInstruction {
    pub fn parse(input: &str) -> IResult<&str, AssemblyInstruction> {
        alt((
            alt((
                AssemblyInstruction::parse_nop,
                AssemblyInstruction::parse_mv,
                AssemblyInstruction::parse_memr,
                AssemblyInstruction::parse_memw,
                AssemblyInstruction::parse_msrl,
                AssemblyInstruction::parse_msrh,
                AssemblyInstruction::parse_li,
                AssemblyInstruction::parse_zero,
                AssemblyInstruction::parse_rtwl,
            )),
            alt((
                AssemblyInstruction::parse_rtwh,
                AssemblyInstruction::parse_rtrl,
                AssemblyInstruction::parse_rtrh,
                AssemblyInstruction::parse_add,
                AssemblyInstruction::parse_sub,
                AssemblyInstruction::parse_nand,
                AssemblyInstruction::parse_xor,
                AssemblyInstruction::parse_addi,
                AssemblyInstruction::parse_inc,
            )),
            alt((
                AssemblyInstruction::parse_dec,
                AssemblyInstruction::parse_neg,
                AssemblyInstruction::parse_pjmp,
                AssemblyInstruction::parse_jmp,
                AssemblyInstruction::parse_jal,
                AssemblyInstruction::parse_ret,
                AssemblyInstruction::parse_jcr,
                AssemblyInstruction::parse_jzr,
                AssemblyInstruction::parse_jnr,
            )),
            alt((
                AssemblyInstruction::parse_jltr,
                AssemblyInstruction::parse_spsl,
                AssemblyInstruction::parse_spsh,
                AssemblyInstruction::parse_push,
                AssemblyInstruction::parse_pull,
                AssemblyInstruction::parse_peek,
                AssemblyInstruction::parse_spof,
                AssemblyInstruction::parse_hlt
            )),
        ))(input)
    }

    fn parse_nop(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("NOP"), |_| AssemblyInstruction::NOP)(input)
    }

    fn parse_mv(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("MV"),
            multispace1,
            AssemblyRegister::parse,
            tag(","),
            multispace0,
            AssemblyRegister::parse
        )), |(_, _, dst, _, _, src)| AssemblyInstruction::MV { src, dst })(input)
    }

    fn parse_memr(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("MEMR"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, dst)| AssemblyInstruction::MEMR { dst })(input)
    }

    fn parse_memw(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("MEMW"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, src)| AssemblyInstruction::MEMW { src })(input)
    }

    fn parse_msrl(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("MSRL"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, src)| AssemblyInstruction::MSRL { src })(input)
    }

    fn parse_msrh(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("MSRH"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, src)| AssemblyInstruction::MSRH { src })(input)
    }

    fn parse_li(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("LI"),
            multispace1,
            AssemblyRegister::parse,
            multispace0,
            tag(","),
            multispace0,
            hex_u8,
        )), |(_, _, dst, _, _, _, value)| AssemblyInstruction::LI { dst, value })(input)
    }

    fn parse_zero(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("ZERO"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, dst)| AssemblyInstruction::ZERO { dst })(input)
    }

    fn parse_rtwl(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("RTWL"), |_| AssemblyInstruction::RTWL)(input)
    }

    fn parse_rtwh(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("RTWH"), |_| AssemblyInstruction::RTWH)(input)
    }

    fn parse_rtrl(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("RTRL"), |_| AssemblyInstruction::RTRL)(input)
    }

    fn parse_rtrh(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("RTRH"), |_| AssemblyInstruction::RTRH)(input)
    }

    fn parse_add(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("ADD"),
            multispace1,
            AssemblyRegister::parse,
            tag(","),
            multispace0,
            AssemblyRegister::parse
        )), |(_, _, acc, _, _, val)| AssemblyInstruction::ADD { acc, val })(input)
    }

    fn parse_sub(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("SUB"),
            multispace1,
            AssemblyRegister::parse,
            tag(","),
            multispace0,
            AssemblyRegister::parse
        )), |(_, _, acc, _, _, val)| AssemblyInstruction::SUB { acc, val })(input)
    }

    fn parse_nand(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("NAND"),
            multispace1,
            AssemblyRegister::parse,
            tag(","),
            multispace0,
            AssemblyRegister::parse
        )), |(_, _, acc, _, _, val)| AssemblyInstruction::NAND { acc, val })(input)
    }

    fn parse_xor(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("XOR"),
            multispace1,
            AssemblyRegister::parse,
            tag(","),
            multispace0,
            AssemblyRegister::parse
        )), |(_, _, acc, _, _, val)| AssemblyInstruction::XOR { acc, val })(input)
    }

    fn parse_addi(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("ADDI"),
            multispace1,
            AssemblyRegister::parse,
            multispace0,
            tag(","),
            multispace0,
            hex_u8
        )), |(_, _, dst, _, _, _, value)| AssemblyInstruction::ADDI { dst, value })(input)
    }

    fn parse_inc(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("INC"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, dst)| AssemblyInstruction::INC { dst })(input)
    }

    fn parse_dec(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("DEC"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, dst)| AssemblyInstruction::DEC { dst })(input)
    }

    fn parse_neg(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("NEG"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, dst)| AssemblyInstruction::NEG { dst })(input)
    }

    fn parse_pjmp(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("PJMP"),
            multispace1,
            AbsoluteAddress::parse,
        )), |(_, _, address)| AssemblyInstruction::PJMP { address })(input)
    }

    fn parse_jmp(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("JMP"), |_| AssemblyInstruction::JMP)(input)
    }

    fn parse_jal(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("JAL"), |_| AssemblyInstruction::JAL)(input)
    }

    fn parse_ret(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("RET"), |_| AssemblyInstruction::RET)(input)
    }

    fn parse_jcr(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("JCR"),
            multispace1,
            Offset::parse,
        )), |(_, _, offset)| AssemblyInstruction::JCR { offset })(input)
    }

    fn parse_jzr(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("JZR"),
            multispace1,
            Offset::parse,
        )), |(_, _, offset)| AssemblyInstruction::JZR { offset })(input)
    }

    fn parse_jnr(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("JNR"),
            multispace1,
            Offset::parse,
        )), |(_, _, offset)| AssemblyInstruction::JNR { offset })(input)
    }

    fn parse_jltr(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("JLTR"),
            multispace1,
            Offset::parse,
        )), |(_, _, offset)| AssemblyInstruction::JLTR { offset })(input)
    }

    fn parse_spsl(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("SPSL"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, src)| AssemblyInstruction::SPSL { src })(input)
    }

    fn parse_spsh(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("SPSH"),
            multispace1,
            AssemblyRegister::parse,
        )), |(_, _, src)| AssemblyInstruction::SPSH { src })(input)
    }

    fn parse_push(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("PUSH"), |_| AssemblyInstruction::PUSH)(input)
    }

    fn parse_pull(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("PULL"), |_| AssemblyInstruction::PULL)(input)
    }

    fn parse_peek(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("PEEK"), |_| AssemblyInstruction::PEEK)(input)
    }

    fn parse_spof(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tuple((
            tag("SPOF"),
            multispace1,
            nom::character::complete::u8,
        )), |(_, _, offset)| AssemblyInstruction::SPOF { offset })(input)
    }

    fn parse_hlt(input: &str) -> IResult<&str, AssemblyInstruction> {
        map(tag("HLT"), |_| AssemblyInstruction::HLT)(input)
    }
}

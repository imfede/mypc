use crate::assemble::assembly_instruction::{AbsoluteAddress, AssemblyInstruction, Offset};
use crate::assemble::assembly_line::{AssemblyLine, Label};
use crate::constants::machine_instruction::MachineInstruction;
use eyre::{bail, Report};

pub enum IntermediateElement {
    Label(Label),
    Value(u8),
    Instruction(MachineInstruction),
}

impl IntermediateElement {
    pub fn content_length(&self) -> u16 {
        match self {
            IntermediateElement::Label(Label::Absolute(_)) => 2,
            IntermediateElement::Label(Label::Relative(_)) => 1,
            IntermediateElement::Value(_) => 1,
            IntermediateElement::Instruction(_) => 1,
        }
    }
}

pub struct IntermediateAssemblyLine {
    pub assembly: IntermediateElement,
    pub label: Option<Label>,
}

impl IntermediateAssemblyLine {
    fn new(label: Option<Label>, assembly: IntermediateElement) -> Self {
        IntermediateAssemblyLine { assembly, label }
    }

    fn instruction(label: Option<Label>, instruction: MachineInstruction) -> Self {
        IntermediateAssemblyLine { assembly: IntermediateElement::Instruction(instruction), label }
    }

    fn value(value: u8) -> Self {
        IntermediateAssemblyLine::new(None, IntermediateElement::Value(value))
    }

    fn label_absolute(name: String) -> Self {
        IntermediateAssemblyLine::new(None, IntermediateElement::Label(Label::Absolute(name)))
    }

    fn label_relative(name: String) -> Self {
        IntermediateAssemblyLine::new(None, IntermediateElement::Label(Label::Relative(name)))
    }
}

impl IntermediateAssemblyLine {
    fn map(assembly: AssemblyLine) -> Vec<IntermediateAssemblyLine> {
        match assembly.instruction {
            AssemblyInstruction::NOP => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::NOP)],
            AssemblyInstruction::MV { dst, src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::MV { dst: dst.into(), src: src.into() })],
            AssemblyInstruction::MEMR { dst } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::MEMR { dst: dst.into() })],
            AssemblyInstruction::MEMW { src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::MEMW { src: src.into() })],
            AssemblyInstruction::MSRL { src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::MSRL { src: src.into() })],
            AssemblyInstruction::MSRH { src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::MSRH { src: src.into() })],
            AssemblyInstruction::LI { dst, value } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::LI { dst: dst.into() }),
                IntermediateAssemblyLine::value(value),
            ],
            AssemblyInstruction::ZERO { dst } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::ZERO { dst: dst.into() })],
            AssemblyInstruction::RTWL => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::RTWL)],
            AssemblyInstruction::RTWH => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::RTWH)],
            AssemblyInstruction::RTRL => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::RTRL)],
            AssemblyInstruction::RTRH => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::RTRH)],
            AssemblyInstruction::ADD { acc, val } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::ADD { acc: acc.into(), val: val.into() })],
            AssemblyInstruction::SUB { acc, val } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::SUB { acc: acc.into(), val: val.into() })],
            AssemblyInstruction::NAND { acc, val } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::NAND { acc: acc.into(), val: val.into() })],
            AssemblyInstruction::XOR { acc, val } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::XOR { acc: acc.into(), val: val.into() })],
            AssemblyInstruction::ADDI { dst, value } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::ADDI { dst: dst.into() }),
                IntermediateAssemblyLine::value(value),
            ],
            AssemblyInstruction::INC { dst } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::INC { dst: dst.into() })],
            AssemblyInstruction::DEC { dst } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::DEC { dst: dst.into() })],
            AssemblyInstruction::NEG { dst } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::NEG { dst: dst.into() })],
            AssemblyInstruction::PJMP { address: AbsoluteAddress::Label { name } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::PJMP),
                IntermediateAssemblyLine::label_absolute(name),
            ],
            AssemblyInstruction::PJMP { address: AbsoluteAddress::HardCoded { address } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::PJMP),
                IntermediateAssemblyLine::value((address >> 8) as u8),
                IntermediateAssemblyLine::value((address & 0xFF) as u8),
            ],
            AssemblyInstruction::JMP => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JMP)],
            AssemblyInstruction::JAL => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JAL)],
            AssemblyInstruction::RET => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::RET)],
            AssemblyInstruction::JCR { offset: Offset::Label { name } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JCR),
                IntermediateAssemblyLine::label_relative(name),
            ],
            AssemblyInstruction::JCR { offset: Offset::HardCoded { offset } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JCR),
                IntermediateAssemblyLine::value(offset as u8),
            ],
            AssemblyInstruction::JZR { offset: Offset::Label { name } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JZR),
                IntermediateAssemblyLine::label_relative(name),
            ],
            AssemblyInstruction::JZR { offset: Offset::HardCoded { offset } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JZR),
                IntermediateAssemblyLine::value(offset as u8),
            ],
            AssemblyInstruction::JNR { offset: Offset::Label { name } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JNR),
                IntermediateAssemblyLine::label_relative(name),
            ],
            AssemblyInstruction::JNR { offset: Offset::HardCoded { offset } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JNR),
                IntermediateAssemblyLine::value(offset as u8),
            ],
            AssemblyInstruction::JLTR { offset: Offset::Label { name } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JLTR),
                IntermediateAssemblyLine::label_relative(name),
            ],
            AssemblyInstruction::JLTR { offset: Offset::HardCoded { offset } } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::JLTR),
                IntermediateAssemblyLine::value(offset as u8),
            ],
            AssemblyInstruction::SPSL { src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::SPSL { src: src.into() })],
            AssemblyInstruction::SPSH { src } => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::SPSH { src: src.into() })],
            AssemblyInstruction::PUSH => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::PUSH)],
            AssemblyInstruction::PULL => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::PULL)],
            AssemblyInstruction::PEEK => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::PEEK)],
            AssemblyInstruction::SPOF { offset } => vec![
                IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::SPOF),
                IntermediateAssemblyLine::value(offset),
            ],
            AssemblyInstruction::HLT => vec![IntermediateAssemblyLine::instruction(assembly.label, MachineInstruction::HLT)],
        }
    }
}

pub struct IntermediateAssembly(pub Vec<IntermediateAssemblyLine>);

impl TryFrom<Vec<AssemblyLine>> for IntermediateAssembly {
    type Error = Report;

    fn try_from(input: Vec<AssemblyLine>) -> Result<Self, Report> {
        let vec: Vec<_> = input.into_iter().flat_map(IntermediateAssemblyLine::map).collect();

        if vec.len() > u16::MAX as usize {
            bail!("Assembly too long {}", vec.len());
        }

        Ok(IntermediateAssembly(vec))
    }
}

impl IntermediateAssembly {
    pub fn len(&self) -> usize {
        self.0.iter().map(|x| x.assembly.content_length()).fold(0, |acc, x| acc + (x as usize))
    }
}
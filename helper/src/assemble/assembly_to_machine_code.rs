use crate::assemble::assembly_instruction::AssemblyInstruction;
use crate::constants::machine_instruction::MachineInstruction;
use std::collections::HashMap;

pub fn to_machine_code(instruction: AssemblyInstruction, instructions_value: &HashMap<MachineInstruction, u8>) -> Vec<u8> {
    match instruction {
        AssemblyInstruction::NOP => vec![instructions_value[&MachineInstruction::NOP]],
        AssemblyInstruction::MV { src, dst } => vec![instructions_value[&MachineInstruction::MV { src: src.into(), dst: dst.into() }]],
        AssemblyInstruction::MEMR { dst } => vec![instructions_value[&MachineInstruction::MEMR { dst: dst.into() }]],
        AssemblyInstruction::MEMW { src } => vec![instructions_value[&MachineInstruction::MEMW { src: src.into() }]],
        AssemblyInstruction::MSRL { src } => vec![instructions_value[&MachineInstruction::MSRL { src: src.into() }]],
        AssemblyInstruction::MSRH { src } => vec![instructions_value[&MachineInstruction::MSRH { src: src.into() }]],
        AssemblyInstruction::LI { dst, value } => vec![instructions_value[&MachineInstruction::LI { dst: dst.into() }], value],
        AssemblyInstruction::ZERO { dst } => vec![instructions_value[&MachineInstruction::ZERO { dst: dst.into() }]],
        AssemblyInstruction::RTWL => vec![instructions_value[&MachineInstruction::RTWL]],
        AssemblyInstruction::RTWH => vec![instructions_value[&MachineInstruction::RTWH]],
        AssemblyInstruction::RTRL => vec![instructions_value[&MachineInstruction::RTRL]],
        AssemblyInstruction::RTRH => vec![instructions_value[&MachineInstruction::RTRH]],
        AssemblyInstruction::ADD { acc, val } => vec![instructions_value[&MachineInstruction::ADD { acc: acc.into(), val: val.into() }]],
        AssemblyInstruction::SUB { acc, val } => vec![instructions_value[&MachineInstruction::SUB { acc: acc.into(), val: val.into() }]],
        AssemblyInstruction::NAND { acc, val } => vec![instructions_value[&MachineInstruction::NAND { acc: acc.into(), val: val.into() }]],
        AssemblyInstruction::XOR { acc, val } => vec![instructions_value[&MachineInstruction::XOR { acc: acc.into(), val: val.into() }]],
        AssemblyInstruction::ADDI { dst, value } => vec![instructions_value[&MachineInstruction::ADDI { dst: dst.into() }], value],
        AssemblyInstruction::INC { dst } => vec![instructions_value[&MachineInstruction::INC { dst: dst.into() }]],
        AssemblyInstruction::DEC { dst } => vec![instructions_value[&MachineInstruction::DEC { dst: dst.into() }]],
        AssemblyInstruction::NEG { dst } => vec![instructions_value[&MachineInstruction::NEG { dst: dst.into() }]],
        AssemblyInstruction::PJMP { address } => vec![instructions_value[&MachineInstruction::PJMP], (address >> 8) as u8, (address & 0xFF) as u8],
        AssemblyInstruction::JMP => vec![instructions_value[&MachineInstruction::JMP]],
        AssemblyInstruction::JAL => vec![instructions_value[&MachineInstruction::JAL]],
        AssemblyInstruction::RET => vec![instructions_value[&MachineInstruction::RET]],
        AssemblyInstruction::JCR { offset } => vec![instructions_value[&MachineInstruction::JCR], offset as u8],
        AssemblyInstruction::JZR { offset } => vec![instructions_value[&MachineInstruction::JZR], offset as u8],
        AssemblyInstruction::JNR { offset } => vec![instructions_value[&MachineInstruction::JNR], offset as u8],
        AssemblyInstruction::JLTR { offset } => vec![instructions_value[&MachineInstruction::JLTR], offset as u8],
        AssemblyInstruction::SPSL { src } => vec![instructions_value[&MachineInstruction::SPSL { src: src.into() }]],
        AssemblyInstruction::SPSH { src } => vec![instructions_value[&MachineInstruction::SPSH { src: src.into() }]],
        AssemblyInstruction::PUSH => vec![instructions_value[&MachineInstruction::PUSH]],
        AssemblyInstruction::PULL => vec![instructions_value[&MachineInstruction::PULL]],
        AssemblyInstruction::PEEK => vec![instructions_value[&MachineInstruction::PEEK]],
        AssemblyInstruction::SPOF { offset } => vec![instructions_value[&MachineInstruction::SPOF], offset],
        AssemblyInstruction::HLT => vec![instructions_value[&MachineInstruction::HLT]],
    }
}
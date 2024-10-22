use crate::constants::control_line::ControlLine;
use crate::constants::control_word::ControlWord;
use crate::constants::flag::{Flag, Flags};
use crate::constants::general_register::GeneralRegister;
use crate::starts_with::StartsWith;
use std::hash::Hash;

#[allow(unused, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MachineInstruction {
    /// Move, move the content of a general register to another general register.
    MV {
        dst: GeneralRegister,
        src: GeneralRegister,
    },
    /// Memory Read, read a word from memory (the address is selected by setting the address registers with MSRL and MSRH)
    MEMR { dst: GeneralRegister },
    /// Memory Write, write a word to memory (the address is selected by setting the address registers with MSRL and MSRH)
    MEMW { src: GeneralRegister },
    /// Memory Set Register Low, put the content of the selected register in the RAM address register low.
    MSRL { src: GeneralRegister },
    /// Memory Set Register High, put the content of the selected register in the RAM address register high.
    MSRH { src: GeneralRegister },
    /// Load Immediate, read the next instruction as data, and store it into the selected register.
    LI { dst: GeneralRegister },
    /// Zero, set the content of the selected register to 0x00.
    ZERO { dst: GeneralRegister },
    /// Return Write Low, write the contents of the RET LOW register to RAM, at the address selected by the MSRL and MSRH instructions.
    RTWL,
    /// Return Write High, Write the contents of the RET HIGH register to RAM, at the address selected by the MSRL and MSRH instructions.
    RTWH,
    /// Return Read Low, put the content of the RAM at the address selected by the MSRL and MSRH instructions into the RET LOW register.
    RTRL,
    /// Return Read High, put the content of the RAM at the address selected by the MSRL and MSRH instructions into the RET HIGH register.
    RTRH,
    /// Add, add the content of two general registers, and store the result in the first register.
    ADD { acc: GeneralRegister, val: GeneralRegister },
    /// Subtract, subtract the content of two general registers, and store the result in the first register.
    SUB { acc: GeneralRegister, val: GeneralRegister },
    /// NAND, bitwise NAND the content of two general registers, and store the result in the first register.
    NAND { acc: GeneralRegister, val: GeneralRegister },
    /// XOR, bitwise XOR the content of two general registers, and store the result in the first register.
    XOR { acc: GeneralRegister, val: GeneralRegister },
    /// Add Immediate, interpret the next instruction as an 8-bit immediate value, and add it to the content of the selected register.
    ADDI { dst: GeneralRegister },
    /// Increment, increment the content of the selected register by 1.
    INC { dst: GeneralRegister },
    /// Decrement, decrement the content of the selected register by 1.
    DEC { dst: GeneralRegister },
    /// Negate, negate the content of the selected register.
    NEG { dst: GeneralRegister },
    /// Prepare Jump, read the next two instructions as a 16-bit immediate value (the first byte is the most significant), and set the JMP registers to it.
    PJMP,
    /// Jump, set the IP to the value of the JMP registers.
    JMP,
    /// Jump And Link, set the IP to the value of the JMP registers, and set the RET registers to the old IP.
    JAL,
    /// Return, set the IP to the value of the RET registers.
    RET,
    /// Jump if Carry, read the next instruction as a signed 8-bit integer, and jump to that offset if the carry flag is set
    JCR,
    /// Jump if Zero, read the next instruction as a signed 8-bit integer, and jump to that offset if the zero flag is set
    JZR,
    /// Jump if Negative, read the next instruction as a signed 8-bit integer, and jump to that offset if the negative flag is set
    JNR,
    /// Jump if A1 < A2, read the next instruction as a signed 8-bit integer, and jump to that offset if the A1LTA2 flag is set
    JLTR,
    /// Stack Pointer Set Low, set the stack pointer register low to the value contained in the selected register
    SPSL { src: GeneralRegister },
    /// Stack Poiner Set High, set the stack pointer register low to the value contained in the selected register
    SPSH { src: GeneralRegister },
    /// Push, set the memory registers to the values contained in the stack pointer registers, and advance (decrement, as the stack grows towards lesser addresses) the stack pointer registers
    PUSH,
    /// Pull, retreat (increment, as the stack grows toward lesser addresses) the stack pointer registers and sed the memory registers to the value contained in the stack registers
    PULL,
    /// Peek, set the memory registers to the content of the stack registers
    PEEK,
    /// Stack Pointer Offset, read the next instruction as a 8-bit offset and set the memory registries to the content of the stack pointers plus the offset.
    SPOF,
    /// Halt, stops the clock.
    HLT,
    /// No Operation, do nothing.
    NOP,
}

impl From<u8> for MachineInstruction {
    fn from(value: u8) -> Self {
        match value {
            i if i.binary_representation_starts_with("00_00") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::MV { dst, src }
            }
            i if i.binary_representation_starts_with("00_01_00") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::MEMR { dst }
            }
            i if i.binary_representation_starts_with("00_01_01") => {
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::MEMW { src }
            }
            i if i.binary_representation_starts_with("00_01_10") => {
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::MSRL { src }
            }
            i if i.binary_representation_starts_with("00_01_11") => {
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::MSRH { src }
            }
            i if i.binary_representation_starts_with("00_10_00") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::LI { dst }
            }
            i if i.binary_representation_starts_with("00_10_01") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::ZERO { dst }
            }
            0b00_10_11_00 => MachineInstruction::RTWL,
            0b00_10_11_01 => MachineInstruction::RTWH,
            0b00_10_11_10 => MachineInstruction::RTRL,
            0b00_10_11_11 => MachineInstruction::RTRH,
            i if i.binary_representation_starts_with("01_00") => {
                let acc = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let val = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::ADD { acc, val }
            }
            i if i.binary_representation_starts_with("01_01") => {
                let acc = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let val = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::SUB { acc, val }
            }
            i if i.binary_representation_starts_with("01_10") => {
                let acc = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let val = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::NAND { acc, val }
            }
            i if i.binary_representation_starts_with("01_11") => {
                let acc = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let val = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::XOR { acc, val }
            }
            i if i.binary_representation_starts_with("10_00_00") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::ADDI { dst }
            }
            i if i.binary_representation_starts_with("10_00_01") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::INC { dst }
            }
            i if i.binary_representation_starts_with("10_00_10") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::DEC { dst }
            }
            i if i.binary_representation_starts_with("10_00_11") => {
                let dst = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::NEG { dst }
            }
            0b11_00_00_00 => MachineInstruction::PJMP,
            0b11_00_00_01 => MachineInstruction::JMP,
            0b11_00_00_10 => MachineInstruction::JAL,
            0b11_00_00_11 => MachineInstruction::RET,
            0b11_00_01_00 => MachineInstruction::JCR,
            0b11_00_01_01 => MachineInstruction::JZR,
            0b11_00_01_10 => MachineInstruction::JNR,
            0b11_00_01_11 => MachineInstruction::JLTR,
            i if i.binary_representation_starts_with("11_11_00") => {
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::SPSL { src }
            }
            i if i.binary_representation_starts_with("11_11_01") => {
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                MachineInstruction::SPSH { src }
            }
            0b11_11_10_00 => MachineInstruction::PUSH,
            0b11_11_10_01 => MachineInstruction::PULL,
            0b11_11_10_10 => MachineInstruction::PEEK,
            0b11_11_10_11 => MachineInstruction::SPOF,
            0b11_11_11_11 => MachineInstruction::HLT,
            _ => MachineInstruction::NOP,
        }
    }
}

pub fn steps(instruction: MachineInstruction, flags: Flags) -> [ControlWord; 16] {
    let instruction_steps = match instruction {
        MachineInstruction::NOP => vec![],
        MachineInstruction::MV { src, dst } => vec![src.register_out() | dst.register_in()],
        MachineInstruction::MEMR { dst } => {
            vec![dst.register_in() | [ControlLine::MIS, ControlLine::MO].into()]
        }
        MachineInstruction::MEMW { src } => {
            vec![src.register_out() | [ControlLine::MIS, ControlLine::MI].into()]
        }
        MachineInstruction::MSRL { src } => vec![src.register_out() | [ControlLine::WME].into()],
        MachineInstruction::MSRH { src } => {
            vec![src.register_out() | [ControlLine::WME, ControlLine::WMS].into()]
        }
        MachineInstruction::LI { dst } => vec![
            dst.register_in() | [ControlLine::MO].into(),
            [ControlLine::IPA].into(),
        ],
        MachineInstruction::ZERO { dst } => vec![dst.register_in()],
        MachineInstruction::RTWL => vec![[ControlLine::MIS, ControlLine::MI, ControlLine::RETE].into()],
        MachineInstruction::RTWH => vec![[
            ControlLine::MIS,
            ControlLine::MI,
            ControlLine::RETE,
            ControlLine::RETS,
        ]
            .into()],
        MachineInstruction::RTRL => vec![[
            ControlLine::MIS,
            ControlLine::MO,
            ControlLine::RETE,
            ControlLine::RETI,
        ]
            .into()],
        MachineInstruction::RTRH => vec![[
            ControlLine::MIS,
            ControlLine::MO,
            ControlLine::RETE,
            ControlLine::RETI,
            ControlLine::RETS,
        ]
            .into()],
        MachineInstruction::ADD { acc, val } => vec![
            acc.register_out() | [ControlLine::A1I].into(),
            val.register_out() | [ControlLine::A2I].into(),
            acc.register_in() | [ControlLine::AO].into(),
        ],
        MachineInstruction::SUB { acc, val } => vec![
            acc.register_out() | [ControlLine::A1I].into(),
            val.register_out() | [ControlLine::A2I].into(),
            acc.register_in() | [ControlLine::AO, ControlLine::AOPL].into(),
        ],
        MachineInstruction::NAND { acc, val } => vec![
            acc.register_out() | [ControlLine::A1I].into(),
            val.register_out() | [ControlLine::A2I].into(),
            acc.register_in() | [ControlLine::AO, ControlLine::AOPH].into(),
        ],
        MachineInstruction::XOR { acc, val } => vec![
            acc.register_out() | [ControlLine::A1I].into(),
            val.register_out() | [ControlLine::A2I].into(),
            acc.register_in() | [ControlLine::AO, ControlLine::AOPL, ControlLine::AOPH].into(),
        ],
        MachineInstruction::ADDI { dst } => vec![
            [ControlLine::MO, ControlLine::A2I].into(),
            dst.register_out() | [ControlLine::IPA, ControlLine::A1I].into(),
            dst.register_in() | [ControlLine::AO].into(),
        ],
        MachineInstruction::INC { dst } => vec![
            [ControlLine::ONEO, ControlLine::A2I].into(),
            dst.register_out() | [ControlLine::A1I].into(),
            dst.register_in() | [ControlLine::AO].into(),
        ],
        MachineInstruction::DEC { dst } => vec![
            [ControlLine::ONEO, ControlLine::A2I].into(),
            dst.register_out() | [ControlLine::A1I].into(),
            dst.register_in() | [ControlLine::AO, ControlLine::AOPL].into(),
        ],
        MachineInstruction::NEG { dst } => vec![
            [ControlLine::A1I].into(),
            dst.register_out() | [ControlLine::A2I].into(),
            dst.register_in() | [ControlLine::AO, ControlLine::AOPL, ControlLine::CI].into(),
        ],
        MachineInstruction::PJMP => vec![
            [ControlLine::MO, ControlLine::JMPE, ControlLine::JMPI, ControlLine::JMPS].into(),
            [ControlLine::IPA].into(),
            [ControlLine::MO, ControlLine::JMPE, ControlLine::JMPI].into(),
            [ControlLine::IPA].into(),
        ],
        MachineInstruction::JMP => vec![
            [ControlLine::JMPE, ControlLine::IPE].into(),
            [ControlLine::JMPE, ControlLine::JMPS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        MachineInstruction::JAL => vec![
            [ControlLine::IPE, ControlLine::IPO, ControlLine::RETE, ControlLine::RETI].into(),
            [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::RETE, ControlLine::RETI, ControlLine::RETS].into(),
            [ControlLine::JMPE, ControlLine::IPE].into(),
            [ControlLine::JMPE, ControlLine::JMPS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        MachineInstruction::RET => vec![
            [ControlLine::RETE, ControlLine::IPE].into(),
            [ControlLine::RETE, ControlLine::RETS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        MachineInstruction::JCR => {
            vec![
                if flags.has(Flag::CO) {
                    [ControlLine::A1I, ControlLine::IPE, ControlLine::IPO].into()
                } else {
                    [ControlLine::IPA, ControlLine::MRST].into()
                },
                [ControlLine::MO, ControlLine::A2I].into(),
                [ControlLine::AO, ControlLine::CI, ControlLine::IPE].into(),
                if flags.has(Flag::CO) {
                    if flags.has(Flag::A2G1) {
                        [ControlLine::FFO, ControlLine::A2I].into()
                    } else {
                        [ControlLine::ONEO, ControlLine::A2I].into()
                    }
                } else {
                    [ControlLine::MRST].into()
                },
                [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::A1I].into(),
                [ControlLine::AO, ControlLine::IPE, ControlLine::IPS].into(),
            ]
        }
        MachineInstruction::JZR => {
            vec![
                if flags.has(Flag::FZ) {
                    [ControlLine::A1I, ControlLine::IPE, ControlLine::IPO].into()
                } else {
                    [ControlLine::IPA, ControlLine::MRST].into()
                },
                [ControlLine::MO, ControlLine::A2I].into(),
                [ControlLine::AO, ControlLine::CI, ControlLine::IPE].into(),
                if flags.has(Flag::CO) {
                    if flags.has(Flag::A2G1) {
                        [ControlLine::FFO, ControlLine::A2I].into()
                    } else {
                        [ControlLine::ONEO, ControlLine::A2I].into()
                    }
                } else {
                    [ControlLine::MRST].into()
                },
                [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::A1I].into(),
                [ControlLine::AO, ControlLine::IPE, ControlLine::IPS].into(),
            ]
        }
        MachineInstruction::JNR => {
            vec![
                if flags.has(Flag::NEG) {
                    [ControlLine::A1I, ControlLine::IPE, ControlLine::IPO].into()
                } else {
                    [ControlLine::IPA, ControlLine::MRST].into()
                },
                [ControlLine::MO, ControlLine::A2I].into(),
                [ControlLine::AO, ControlLine::CI, ControlLine::IPE].into(),
                if flags.has(Flag::CO) {
                    if flags.has(Flag::A2G1) {
                        [ControlLine::FFO, ControlLine::A2I].into()
                    } else {
                        [ControlLine::ONEO, ControlLine::A2I].into()
                    }
                } else {
                    [ControlLine::MRST].into()
                },
                [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::A1I].into(),
                [ControlLine::AO, ControlLine::IPE, ControlLine::IPS].into(),
            ]
        }
        MachineInstruction::JLTR => {
            vec![
                if flags.has(Flag::A2G1) {
                    [ControlLine::A1I, ControlLine::IPE, ControlLine::IPO].into()
                } else {
                    [ControlLine::IPA, ControlLine::MRST].into()
                },
                [ControlLine::MO, ControlLine::A2I].into(),
                [ControlLine::AO, ControlLine::CI, ControlLine::IPE].into(),
                if flags.has(Flag::CO) {
                    if flags.has(Flag::A2G1) {
                        [ControlLine::FFO, ControlLine::A2I].into()
                    } else {
                        [ControlLine::ONEO, ControlLine::A2I].into()
                    }
                } else {
                    [ControlLine::MRST].into()
                },
                [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::A1I].into(),
                [ControlLine::AO, ControlLine::IPE, ControlLine::IPS].into(),
            ]
        }
        MachineInstruction::SPSL { src } => vec![
            src.register_out() | [ControlLine::SPE, ControlLine::SPI].into(),
        ],
        MachineInstruction::SPSH { src } => vec![
            src.register_out() | [ControlLine::SPE, ControlLine::SPI, ControlLine::SPS].into()
        ],
        MachineInstruction::PUSH => vec![
            [ControlLine::SPE, ControlLine::A1I, ControlLine::WME].into(),
            [ControlLine::ONEO, ControlLine::A2I].into(),
            [ControlLine::AO, ControlLine::AOPL, ControlLine::SPE, ControlLine::SPI].into(),
            if flags.has(Flag::A2G1) {
                [].into()
            } else {
                [ControlLine::A2I].into()
            },
            [ControlLine::SPE, ControlLine::SPS, ControlLine::A1I, ControlLine::WME, ControlLine::WMS].into(),
            [ControlLine::AO, ControlLine::AOPL, ControlLine::SPE, ControlLine::SPI, ControlLine::SPS].into(),
        ],
        MachineInstruction::PULL => vec![
            [ControlLine::SPE, ControlLine::A1I].into(),
            [ControlLine::ONEO, ControlLine::A2I].into(),
            [ControlLine::AO, ControlLine::SPE, ControlLine::SPI, ControlLine::WME].into(),
            if flags.has(Flag::CO) {
                [].into()
            } else {
                [ControlLine::A2I].into()
            },
            [ControlLine::SPE, ControlLine::SPS, ControlLine::A1I].into(),
            [ControlLine::AO, ControlLine::SPI, ControlLine::SPE, ControlLine::SPS, ControlLine::WME, ControlLine::WMS].into(),
        ],
        MachineInstruction::PEEK => vec![
            [ControlLine::SPE, ControlLine::WME].into(),
            [ControlLine::SPE, ControlLine::SPS, ControlLine::WME, ControlLine::WMS].into(),
        ],
        MachineInstruction::SPOF => vec![
            [ControlLine::MO, ControlLine::A1I].into(),
            [ControlLine::IPA, ControlLine::SPE, ControlLine::A2I].into(),
            [ControlLine::AO, ControlLine::WME].into(),
            if flags.has(Flag::CO) {
                [ControlLine::ONEO, ControlLine::A2I].into()
            } else {
                [ControlLine::A2I].into()
            },
            [ControlLine::SPE, ControlLine::SPS, ControlLine::A1I].into(),
            [ControlLine::AO, ControlLine::WME, ControlLine::WMS].into(),
        ],
        MachineInstruction::HLT => vec![
            [ControlLine::HLT].into(),
        ]
    };

    #[allow(clippy::get_first)]
    [
        [ControlLine::MO, ControlLine::IRE].into(),
        [ControlLine::IPA].into(),
        instruction_steps
            .get(0)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(1)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(2)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(3)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(4)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(5)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(6)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(7)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(8)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(9)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(10)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(11)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(12)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
        instruction_steps
            .get(13)
            .unwrap_or(&[ControlLine::MRST].into())
            .to_owned(),
    ]
}

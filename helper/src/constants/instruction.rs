use crate::constants::control_line::ControlLine;
use crate::constants::control_word::ControlWord;
use crate::constants::flag::{Flag, Flags};
use crate::constants::general_register::GeneralRegister;
use crate::starts_with::StartsWith;

#[allow(unused, clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// Move, move the content of a general register to another general register.
    MV(GeneralRegister, GeneralRegister),
    /// Memory Read, read a word from memory (the address is selected by setting the address registers with MSRL and MSRH)
    MEMR(GeneralRegister),
    /// Memory Write, write a word to memory (the address is selected by setting the address registers with MSRL and MSRH)
    MEMW(GeneralRegister),
    /// Memory Set Register Low, put the content of the selected register in the RAM address register low.
    MSRL(GeneralRegister),
    /// Memory Set Register High, put the content of the selected register in the RAM address register high.
    MSRH(GeneralRegister),
    /// Load Immediate, read the next instruction as data, and store it into the selected register.
    LI(GeneralRegister),
    /// Zero, set the content of the selected register to 0x00.
    ZERO(GeneralRegister),
    /// Return Write Low, write the contents of the RET LOW register to RAM, at the address selected by the MSRL and MSRH instructions.
    RTWL,
    /// Return Write High, Write the contents of the RET HIGH register to RAM, at the address selected by the MSRL and MSRH instructions.
    RTWH,
    /// Return Read Low, put the content of the RAM at the address selected by the MSRL and MSRH instructions into the RET LOW register.
    RTRL,
    /// Return Read High, put the content of the RAM at the address selected by the MSRL and MSRH instructions into the RET HIGH register.
    RTRH,
    /// Add, add the content of two general registers, and store the result in the first register.
    ADD(GeneralRegister, GeneralRegister),
    /// Subtract, subtract the content of two general registers, and store the result in the first register.
    SUB(GeneralRegister, GeneralRegister),
    /// NAND, bitwise NAND the content of two general registers, and store the result in the first register.
    NAND(GeneralRegister, GeneralRegister),
    /// XOR, bitwise XOR the content of two general registers, and store the result in the first register.
    XOR(GeneralRegister, GeneralRegister),
    /// Add Immediate, interpret the next instruction as an 8-bit immediate value, and add it to the content of the selected register.
    ADDI(GeneralRegister),
    /// Increment, increment the content of the selected register by 1.
    INC(GeneralRegister),
    /// Decrement, decrement the content of the selected register by 1.
    DEC(GeneralRegister),
    /// Negate, negate the content of the selected register.
    NEG(GeneralRegister),
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
    SPSL(GeneralRegister),
    /// Stack Poiner Set High, set the stack pointer register low to the value contained in the selected register
    SPSH(GeneralRegister),
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

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            i if i.binary_representation_starts_with("00_00") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::MV(src, dst)
            }
            i if i.binary_representation_starts_with("00_01_00") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::MEMR(reg)
            }
            i if i.binary_representation_starts_with("00_01_01") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::MEMW(reg)
            }
            i if i.binary_representation_starts_with("00_01_10") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::MSRL(reg)
            }
            i if i.binary_representation_starts_with("00_01_11") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::MSRH(reg)
            }
            i if i.binary_representation_starts_with("00_10_00") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::LI(reg)
            }
            i if i.binary_representation_starts_with("00_10_01") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::ZERO(reg)
            }
            0b00_10_11_00 => Instruction::RTWL,
            0b00_10_11_01 => Instruction::RTWH,
            0b00_10_11_10 => Instruction::RTRL,
            0b00_10_11_11 => Instruction::RTRH,
            i if i.binary_representation_starts_with("01_00") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::ADD(dst, src)
            }
            i if i.binary_representation_starts_with("01_01") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::SUB(dst, src)
            }
            i if i.binary_representation_starts_with("01_10") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::NAND(dst, src)
            }
            i if i.binary_representation_starts_with("01_11") => {
                let dst = GeneralRegister::from((i & 0b00_00_11_00) >> 2);
                let src = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::XOR(dst, src)
            }
            i if i.binary_representation_starts_with("10_00_00") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::ADDI(reg)
            }
            i if i.binary_representation_starts_with("10_00_01") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::INC(reg)
            }
            i if i.binary_representation_starts_with("10_00_10") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::DEC(reg)
            }
            i if i.binary_representation_starts_with("10_00_11") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::NEG(reg)
            }
            0b11_00_00_00 => Instruction::PJMP,
            0b11_00_00_01 => Instruction::JMP,
            0b11_00_00_10 => Instruction::JAL,
            0b11_00_00_11 => Instruction::RET,
            0b11_00_01_00 => Instruction::JCR,
            0b11_00_01_01 => Instruction::JZR,
            0b11_00_01_10 => Instruction::JNR,
            0b11_00_01_11 => Instruction::JLTR,
            i if i.binary_representation_starts_with("11_11_00") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::SPSL(reg)
            }
            i if i.binary_representation_starts_with("11_11_01") => {
                let reg = GeneralRegister::from(i & 0b00_00_00_11);
                Instruction::SPSH(reg)
            }
            0b11_11_10_00 => Instruction::PUSH,
            0b11_11_10_01 => Instruction::PULL,
            0b11_11_10_10 => Instruction::PEEK,
            0b11_11_10_11 => Instruction::SPOF,
            0b11_11_11_11 => Instruction::HLT,
            _ => Instruction::NOP,
        }
    }
}

pub fn steps(instruction: Instruction, flags: Flags) -> [ControlWord; 16] {
    let instruction_steps = match instruction {
        Instruction::NOP => vec![],
        Instruction::MV(src, dst) => vec![src.register_out() | dst.register_in()],
        Instruction::MEMR(reg) => {
            vec![reg.register_in() | [ControlLine::MIS, ControlLine::MO].into()]
        }
        Instruction::MEMW(reg) => {
            vec![reg.register_out() | [ControlLine::MIS, ControlLine::MI].into()]
        }
        Instruction::MSRL(reg) => vec![reg.register_out() | [ControlLine::WME].into()],
        Instruction::MSRH(reg) => {
            vec![reg.register_out() | [ControlLine::WME, ControlLine::WMS].into()]
        }
        Instruction::LI(reg) => vec![
            reg.register_in() | [ControlLine::MO].into(),
            [ControlLine::IPA].into(),
        ],
        Instruction::ZERO(reg) => vec![reg.register_in()],
        Instruction::RTWL => vec![[ControlLine::MIS, ControlLine::MI, ControlLine::RETE].into()],
        Instruction::RTWH => vec![[
            ControlLine::MIS,
            ControlLine::MI,
            ControlLine::RETE,
            ControlLine::RETS,
        ]
            .into()],
        Instruction::RTRL => vec![[
            ControlLine::MIS,
            ControlLine::MO,
            ControlLine::RETE,
            ControlLine::RETI,
        ]
            .into()],
        Instruction::RTRH => vec![[
            ControlLine::MIS,
            ControlLine::MO,
            ControlLine::RETE,
            ControlLine::RETI,
            ControlLine::RETS,
        ]
            .into()],
        Instruction::ADD(dst, src) => vec![
            dst.register_out() | [ControlLine::A1I].into(),
            src.register_out() | [ControlLine::A2I].into(),
            dst.register_in() | [ControlLine::AO].into(),
        ],
        Instruction::SUB(dst, src) => vec![
            dst.register_out() | [ControlLine::A1I].into(),
            src.register_out() | [ControlLine::A2I].into(),
            dst.register_in() | [ControlLine::AO, ControlLine::AOPL].into(),
        ],
        Instruction::NAND(dst, src) => vec![
            dst.register_out() | [ControlLine::A1I].into(),
            src.register_out() | [ControlLine::A2I].into(),
            dst.register_in() | [ControlLine::AO, ControlLine::AOPH].into(),
        ],
        Instruction::XOR(dst, src) => vec![
            dst.register_out() | [ControlLine::A1I].into(),
            src.register_out() | [ControlLine::A2I].into(),
            dst.register_in() | [ControlLine::AO, ControlLine::AOPL, ControlLine::AOPH].into(),
        ],
        Instruction::ADDI(reg) => vec![
            [ControlLine::MO, ControlLine::A2I].into(),
            reg.register_out() | [ControlLine::IPA, ControlLine::A1I].into(),
            reg.register_in() | [ControlLine::AO].into(),
        ],
        Instruction::INC(reg) => vec![
            [ControlLine::ONEO, ControlLine::A2I].into(),
            reg.register_out() | [ControlLine::A1I].into(),
            reg.register_in() | [ControlLine::AO].into(),
        ],
        Instruction::DEC(reg) => vec![
            [ControlLine::ONEO, ControlLine::A2I].into(),
            reg.register_out() | [ControlLine::A1I].into(),
            reg.register_in() | [ControlLine::AO, ControlLine::AOPL].into(),
        ],
        Instruction::NEG(reg) => vec![
            [ControlLine::A1I].into(),
            reg.register_out() | [ControlLine::A2I].into(),
            reg.register_in() | [ControlLine::AO, ControlLine::AOPL, ControlLine::CI].into(),
        ],
        Instruction::PJMP => vec![
            [ControlLine::MO, ControlLine::JMPE, ControlLine::JMPI, ControlLine::JMPS].into(),
            [ControlLine::IPA].into(),
            [ControlLine::MO, ControlLine::JMPE, ControlLine::JMPI].into(),
            [ControlLine::IPA].into(),
        ],
        Instruction::JMP => vec![
            [ControlLine::JMPE, ControlLine::IPE].into(),
            [ControlLine::JMPE, ControlLine::JMPS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        Instruction::JAL => vec![
            [ControlLine::IPE, ControlLine::IPO, ControlLine::RETE, ControlLine::RETI].into(),
            [ControlLine::IPE, ControlLine::IPO, ControlLine::IPS, ControlLine::RETE, ControlLine::RETI, ControlLine::RETS].into(),
            [ControlLine::JMPE, ControlLine::IPE].into(),
            [ControlLine::JMPE, ControlLine::JMPS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        Instruction::RET => vec![
            [ControlLine::RETE, ControlLine::IPE].into(),
            [ControlLine::RETE, ControlLine::RETS, ControlLine::IPE, ControlLine::IPS].into(),
        ],
        Instruction::JCR => {
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
        Instruction::JZR => {
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
        Instruction::JNR => {
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
        Instruction::JLTR => {
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
        Instruction::SPSL(reg) => vec![
            reg.register_out() | [ControlLine::SPE, ControlLine::SPI].into(),
        ],
        Instruction::SPSH(reg) => vec![
            reg.register_out() | [ControlLine::SPE, ControlLine::SPI, ControlLine::SPS].into()
        ],
        Instruction::PUSH => vec![
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
        Instruction::PULL => vec![
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
        Instruction::PEEK => vec![
            [ControlLine::SPE, ControlLine::WME].into(),
            [ControlLine::SPE, ControlLine::SPS, ControlLine::WME, ControlLine::WMS].into(),
        ],
        Instruction::SPOF => vec![
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
        Instruction::HLT => vec![
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

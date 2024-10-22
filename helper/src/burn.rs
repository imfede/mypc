use crate::constants::flag::Flags;
use crate::constants::machine_instruction::{steps, MachineInstruction};
use std::fs;

pub fn burn() {
    let mut rom1 = String::with_capacity(580_000);
    let mut rom2 = String::with_capacity(580_000);

    rom1 += "v3.0 hex bytes plain big-endian\n";
    rom2 += "v3.0 hex bytes plain big-endian\n";

    for rom_address in 0..=0b11_11_11_11_11_11_11_11 {
        let instruction_value = (rom_address & 0b11_11_11_11) as u8;
        let step = (rom_address >> 8) & 0b11_11;
        let flags = ((rom_address >> 12) & 0b11_11) as u8;

        let instruction = MachineInstruction::from(instruction_value);
        let steps = steps(instruction, Flags::from(flags));
        let control_word = steps[step];

        let low_bits = control_word.value() & 0xFF_FF_FF_FF;
        let high_bits = (control_word.value() & 0xFF_FF_FF_FF_00_00_00_00) >> 32;

        rom1 += &format!("{:0>8x}\n", low_bits);
        rom2 += &format!("{:0>8x}\n", high_bits);
    }

    fs::write("rom01.img", rom1).unwrap();
    fs::write("rom02.img", rom2).unwrap();
}

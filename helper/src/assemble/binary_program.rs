use crate::assemble::assembly_line::Label;
use crate::assemble::intermediate_assembly::{IntermediateAssembly, IntermediateElement};
use crate::constants::machine_instruction::MachineInstruction;
use eyre::{bail, Report};
use std::collections::HashMap;

pub struct BinaryProgram(pub Vec<u8>);

impl BinaryProgram {
    fn compute_labels_addresses(assembly: &IntermediateAssembly) -> eyre::Result<HashMap<Label, u16>> {
        let mut addresses: HashMap<Label, u16> = HashMap::new();
        let mut current_index = 0;

        for line in assembly.0.iter() {
            match &line.label {
                Some(label) if addresses.contains_key(label) => bail!("duplicate label: {:?}", label),
                Some(label) => addresses.insert(label.clone(), current_index),
                None => None,
            };

            current_index += line.assembly.content_length();
        }
        Ok(addresses)
    }
}

impl TryFrom<IntermediateAssembly> for BinaryProgram {
    type Error = Report;

    fn try_from(input: IntermediateAssembly) -> Result<Self, Self::Error> {
        let instructions = {
            let mut map: HashMap<MachineInstruction, u8> = (0..=0xFF).map(|value| (MachineInstruction::from(value), value)).collect();
            // Even if all unused instructions are essentially NOPs, for consistency we manually chose one.
            map.insert(MachineInstruction::NOP, 0b11_11_11_10);
            map
        };

        let labels = BinaryProgram::compute_labels_addresses(&input)?;

        let mut buffer: Vec<u8> = Vec::new();

        for line in input.0 {
            match line.assembly {
                IntermediateElement::Instruction(instruction) => buffer.push(instructions[&instruction]),
                IntermediateElement::Value(value) => buffer.push(value),
                IntermediateElement::Label(Label::Relative(name)) => {
                    let address = labels[&Label::Relative(name.clone())];
                    let offset = (address as i32) - (buffer.len() as i32);
                    if offset > i8::MAX as i32 || offset < i8::MIN as i32 {
                        bail!("Offset out of bounds for label {}: {}", name, address);
                    }
                    buffer.push(offset as u8);
                }
                IntermediateElement::Label(Label::Absolute(name)) => {
                    let address = labels[&Label::Absolute(name)];
                    buffer.push((address >> 8) as u8);
                    buffer.push((address & 0xFF) as u8);
                }
            }
        }

        Ok(Self(buffer))
    }
}
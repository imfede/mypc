use crate::assemble::assembly_line::AssemblyLine;
use crate::constants::machine_instruction::MachineInstruction;
use nom::character::complete::multispace1;
use nom::multi::separated_list0;
use nom::IResult;
use std::collections::HashMap;

pub fn assemble(input: String) {
    println!("Assembling:\n-----\n{}\n-----", input);

    let instructions = {
        let mut map: HashMap<MachineInstruction, u8> = (0..=0xFF).map(|value| (MachineInstruction::from(value), value)).collect();
        // Even if all unused instructions are essentially NOPs, for consistency we manually chose one.
        map.insert(MachineInstruction::NOP, 0b11_11_11_10);
        map
    };

    let (rest, assembly) = parse_instructions(&input).unwrap();
    println!("Instructions: {:#?}", assembly);
    println!("Rest: {:#?}", rest);

    // let machine_code: Vec<_> = assembly.into_iter().flat_map(|x| assembly_to_machine_code::to_machine_code(x, &instructions)).collect();
    // println!("Machine code ({}):\n{:?}", machine_code.len(), machine_code);
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<AssemblyLine>> {
    separated_list0(multispace1, AssemblyLine::parse)(input)
}


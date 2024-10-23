use crate::assemble::assembly_line;
use crate::assemble::binary_program::BinaryProgram;
use crate::assemble::intermediate_assembly::IntermediateAssembly;

pub fn assemble(input: &'static str) -> eyre::Result<()> {
    println!("Assembling:\n-----\n{}\n-----", input);

    let (_, assembly) = assembly_line::parse_instructions(input.trim())?;

    let intermediate_assembly = IntermediateAssembly::try_from(assembly)?;

    let assembled_instructions = BinaryProgram::try_from(intermediate_assembly)?;

    println!("Machine code");
    for (index, byte) in assembled_instructions.0.iter().enumerate() {
        println!("{:0>4x}: {:0>8b} ({:0>2x})", index, byte, byte);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() -> eyre::Result<()> {
        let input = "\
        # Start the Fibonacci sequence from 0x01 and 0x00
        LI A, 0x01
        LI B, 0x00

        :loop
            ADD A, B
            # Use D as a temporary register to swap A and B
            MV D, A
            MV A, B
            MV B, D
            JCR .halt
            PJMP :loop
            JMP

        .halt
        HLT";

        let (_, assembly) = assembly_line::parse_instructions(input.trim())?;
        let intermediate_assembly = IntermediateAssembly::try_from(assembly)?;
        let program = BinaryProgram::try_from(intermediate_assembly)?;

        assert_eq!(program.0, vec![0x20, 0x01, 0x21, 0x00, 0x41, 0x0c, 0x01, 0x07, 0xc4, 0x05, 0xc0, 0x00, 0x04, 0xc1, 0xff]);

        Ok(())
    }
}
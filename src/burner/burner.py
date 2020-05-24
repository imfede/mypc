from ..assembler.Instruction import Instruction
from ..assembler.instructions import instructions, getMasked
from ..assembler.lines import MO, IRE, IPA, MRST
import inspect

def findInstruction(instructions, instruction):
    for i in instructions:
        if(i.isThis(instruction)):
            return i
    return Instruction("NOOP", 0, instruction, 0b11_11_11_11, [])

def getStep(instruction, instruction_value, flags, step):
    steps = [lambda: (MO | IRE), lambda: (IPA)] + instruction.steps

    if step >= len(steps):
        return (MRST)

    func = steps[step]
    params = len(inspect.signature(func).parameters)
    if params == 0:
        return func()
    elif params == 1:
        return func(instruction_value)
    elif params == 2:
        return func(instruction_value, flags)

    raise AssertionError(f"Unknown parameter length: {func} {params}")

def writeMemory(memory, instructions):
    for instruction_value in range(256):
        for flags in range(16):
            for step in range(16):
                instruction = findInstruction(instructions, instruction_value)
                memory[(flags << 12) | (step << 8) | instruction_value] = getStep(instruction, instruction_value, flags, step)
    return memory

def get0_31(value):
    return getMasked(0b11111111_11111111_11111111_11111111, value)

def get32_63(value):
    return getMasked(0b11111111_11111111_11111111_11111111 << 32, value)

def main():
    memory = writeMemory({}, instructions)
    with open('rom01.img', 'w') as f1, open('rom02.img', 'w') as f2:
        f1.write("v2.0 raw\n")
        f2.write("v2.0 raw\n")
        for i in range(256*256):
            code1 = hex(get0_31(memory[i]))[2:]
            f1.write(f"{code1}\n")

            code2 = hex(get32_63(memory[i]))[2:]
            f2.write(f"{code2}\n")
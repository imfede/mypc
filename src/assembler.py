from instructions import instructions, getMasked
from sys import argv

def findInstruction(instructions, name):
    for instruction in instructions:
        if instruction.name == name:
            return instruction
    raise AssertionError(f"Unknown instruction: {name}")

def getRegistersNumber(instruction):
    if getMasked(0b1111, instruction.mask) == 0b0000:
        return 2
    if getMasked(0b11, instruction.mask) == 0b00:
        return 1
    return 0

def parseValue(value):
    try:
        val = int(value, 0)
        assert val < 256 and val > -128
        return [val % 256]
    except ValueError:
        if value.startswith("."):
            return [value]
        if value.startswith(":"):
            return [ f"{value}H", f"{value}L" ]
        else:
            print(f"Unknown value: {value}")

registers = {
    "A": 0b00,
    "B": 0b01,
    "C": 0b10,
    "D": 0b11
}

labels = {}
out = []

fname = argv[1]
with open(fname, "r") as f:
    for line in f.readlines():
        line = line.strip()

        # remove line comments
        if line.startswith("#") or line == "":
            continue
        
        # remove inline comments
        line = line.split("#")[0].strip()

        # handle label definition
        if line.startswith(".") or line.startswith(":"):
            assert line not in labels
            labels[line] = len(out)
            continue

        instruction_name = line.split(' ')[0]
        instruction = findInstruction(instructions, instruction_name)

        arguments = list(filter(
            lambda x: x.strip() != '', 
            "".join(line.split(" ")[1:]).split(",")))

        # labels with : count as 2
        count = len(arguments)
        for arg in arguments:
            if arg.startswith(":"):
                count += 1
        assert count == instruction.arity

        register_number = getRegistersNumber(instruction)
        assert register_number <= instruction.arity

        if register_number == 0:
            out.append(instruction.target)
        elif register_number == 1:
            out.append(instruction.target | registers[arguments[0]])
        elif register_number == 2:
            out.append(instruction.target | (registers[arguments[0]] << 2) | registers[arguments[1]] )
        
        for i in range (register_number, len(arguments)):
            out += parseValue(arguments[i])
    
    # resolve labels
    for idx, value in enumerate(out):
        if type(value) == type("string"):
            if value.startswith("."):
                # relative label
                assert value in labels
                offset = (labels[value] - idx)
                assert offset > -128 and offset < 128
                offset = (offset-1) % 256
                out[idx] = offset
            elif value.startswith(":"):
                # absolute value
                assert value[:-1] in labels
                if value[-1] == "L":
                    code = labels[value[:-1]] & 0b11_11_11_11
                    out[idx] = code
                elif value[-1] == "H":
                    code = (labels[value[:-1]] & (0b11_11_11_11 << 8)) >> 8
                    out[idx] = code
                else:
                    print(f"Error: absolute label {value}")
            else:
                raise AssertionError(f"Unknown label type: {value}")

with open("ram.img", "w") as f:
    f.write("v2.0 raw\n")
    for code in out:
        f.write(f"{hex(code)[2:]}\n")
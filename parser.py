from sys import argv

fname = argv[1]

registers = {
    "A": 0b00,
    "B": 0b01,
    "C": 0b10,
    "D": 0b11
}

def parseValue(value):
    try:
        val = int(value, 0)
        assert val < 256 and val > -128
        return val % 256
    except ValueError:
        if value.startswith(".") or value.startswith(":"):
            return value
        else:
            print(f"Unknown value: {value}")

labels = {}

out = []
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

        instruction = line.split(' ')[0]
        if instruction == "MV":
            to = "".join(line.split(' ')[1:]).split(",")[0].strip()
            frm = "".join(line.split(' ')[1:]).split(",")[1].strip()
            out.append((registers[to] << 2) | registers[frm])

        elif instruction == "MEMR":
            to = line.split(' ')[1].strip()
            out.append((0b01_00 << 2) | registers[to])

        elif instruction == "MEMW":
            frm = line.split(' ')[1].strip()
            out.append((0b01_01 << 2) | registers[frm])
        
        elif instruction == "MSRL":
            frm = line.split(' ')[1].strip()
            out.append((0b01_10 << 2) | registers[frm])

        elif instruction == "MSRH":
            frm = line.split(' ')[1].strip()
            out.append((0b01_11 << 2) | registers[frm])

        elif instruction == "LI":
            argvector = "".join(line.split(" ")[1:])
            to = argvector.split(",")[0].strip()
            value = parseValue(argvector.split(",")[1].strip())
            out.append((0b00_10_00 << 2) | registers[to])
            out.append(value)

        elif instruction == "ZERO":
            to = line.split(' ')[1].strip()
            out.append((0b10_01 << 2) | registers[to])

        elif instruction == "ADD":
            a = "".join(line.split(' ')[1:]).split(",")[0].strip()
            b = "".join(line.split(' ')[1:]).split(",")[1].strip()
            out.append((0b01_00 << 4) | (registers[a] << 2) | (registers[b]))

        elif instruction == "SUB":
            a = "".join(line.split(' ')[1:]).split(",")[0].strip()
            b = "".join(line.split(' ')[1:]).split(",")[1].strip()
            out.append((0b01_01 << 4) | (registers[a] << 2) | (registers[b]))

        elif instruction == "NAND":
            a = "".join(line.split(' ')[1:]).split(",")[0].strip()
            b = "".join(line.split(' ')[1:]).split(",")[1].strip()
            out.append((0b01_10 << 4) | (registers[a] << 2) | (registers[b]))

        elif instruction == "XOR":
            a = "".join(line.split(' ')[1:]).split(",")[0].strip()
            b = "".join(line.split(' ')[1:]).split(",")[1].strip()
            out.append((0b01_11 << 4) | (registers[a] << 2) | (registers[b]))

        elif instruction == "ADDI":
            argvector = "".join(line.split(" ")[1:])
            to = argvector.split(",")[0].strip()
            value = parseValue(argvector.split(",")[1].strip())
            out.append((0b10_00_00 << 2) | registers[to])
            out.append(value)

        elif instruction == "INC":
            a = line.split(' ')[1].strip()
            out.append((0b10_00_01 << 2) | registers[a])

        elif instruction == "DEC":
            a = line.split(' ')[1].strip()
            out.append((0b10_00_10 << 2) | registers[to])

        elif instruction == "NEG":
            a = line.split(' ')[1].strip()
            out.append((0b10_00_11 << 2) | registers[to])

        elif instruction == "JMP":
            argvector = "".join(line.split(" ")[1:]).strip()
            if argvector.startswith(":"):
                # abs label
                out.append(0b11_00_00_00)
                out.append(f"{argvector}H")
                out.append(f"{argvector}L")
            else:
                h = parseValue(argvector.split(",")[0].strip())
                l = parseValue(argvector.split(",")[1].strip())
                out.append(0b11_00_00_00)
                out.append(h)
                out.append(l)
        
        elif instruction == "JCR":
            value = parseValue(line.split(" ")[1].strip())
            out.append(0b11_00_01_00)
            out.append(value)

        elif instruction == "JZR":
            value = parseValue(line.split(" ")[1].strip())
            out.append(0b11_00_01_01)
            out.append(value)

        elif instruction == "JNR":
            value = parseValue(line.split(" ")[1].strip())
            out.append(0b11_00_01_10)
            out.append(value)

        elif instruction == "JLTR":
            value = parseValue(line.split(" ")[1].strip())
            out.append(0b11_00_01_11)
            out.append(value)

        elif instruction == "HLT":
            out.append(0xFF)

        else:
            print(f"Unknown: {line}")

with open("ram.img", "w") as f:
    f.write("v2.0 raw\n")
    for idx, value in enumerate(out):
        try:
            code = hex(value)[2:]
            f.write(f"{code}\n")
        except TypeError:
            # its a label
            if value.startswith("."):
                # relative label
                assert value in labels
                offset = (labels[value] - idx)
                assert offset > -128 and offset < 128
                offset = offset-1 % 256
                f.write(f"{offset}\n")
            elif value.startswith(":"):
                assert value[:-1] in labels
                if value[-1] == "L":
                    code = hex(labels[value[:-1]] & 0b11_11_11_11)[2:]
                    f.write(f"{code}\n")
                elif value[-1] == "H":
                    code = hex((labels[value[:-1]] & (0b11_11_11_11 << 8)) >> 8)[2:]
                    f.write(f"{code}\n")
                else:
                    print(f"Error: absolute label {value}")
            else:
                print(f"Unknown label type: {value}")
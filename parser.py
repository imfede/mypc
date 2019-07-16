from sys import argv

fname = argv[1]

registers = {
    "A": 0b00,
    "B": 0b01,
    "C": 0b10,
    "D": 0b11
}

out = []
with open(fname, "r") as f:
    for line in f.readlines():
        line = line.strip()

        # remove line comments
        if line.startswith("#") or line == "":
            continue
        
        # remove inline comments
        line = line.split("#")[0].strip()

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
            value = int(argvector.split(",")[1].strip(), 0)
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
            value = int(argvector.split(",")[1].strip(), 0)
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

        elif instruction == "HLT":
            out.append(0xFF)

        else:
            print(f"Unknown: {line}")

with open("ram.img", "w") as f:
    f.write("v2.0 raw\n")
    for value in out:
        code = hex(value)[2:]
        f.write(f"{code}\n")
        
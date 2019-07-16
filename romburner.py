from collections import defaultdict

# steps
s0 = 0
s1 = 1
s2 = 2
s3 = 3
s4 = 4
s5 = 5
s6 = 6
s7 = 7

# controls
NOOP = 0
RST  = 1 << 0
MRST = 1 << 1
ROE  = 1 << 2
ROL  = 1 << 3
ROH  = 1 << 4
RIE  = 1 << 5
RIL  = 1 << 6
RIH  = 1 << 7
HLT  = 1 << 8

MI   = 1 << 9
WME  = 1 << 10
WMS  = 1 << 11
MIS  = 1 << 12
IPA  = 1 << 13
MO   = 1 << 14
IRE  = 1 << 15

A1I  = 1 << 16
A2I  = 1 << 17
CI   = 1 << 18
AOPL = 1 << 19
AOPH = 1 << 20
AO   = 1 << 21
ZO   = 1 << 22

# instructions

instructions = defaultdict(lambda: [])
memory = defaultdict(lambda: 0)

def demux(value):
    assert value < 4 and value >= 0
    l = value & 0b01
    h = (value & 0b10) >> 1
    return (l, h)

def getMasked(mask, value):
    assert mask != 0
    offset = 0
    while (mask >> offset) & 0b1 == 0:
        offset += 1
    return (value & mask) >> offset

def passIf(value, check):
    return value if check else NOOP

def getStep(instruction, flags, step):
    if step == s0:
        return MO | IRE
    if step == s1:
        return IPA
    if step == s2:
        return NOOP

    if getMasked(0b11_11_00_00, instruction) == 0b00_00:
        # mv between registers
        toL, toH = demux(getMasked(0b1100, instruction))
        frmL, frmH = demux(getMasked(0b0011, instruction))
        if step == s3:
            return (ROE | passIf(ROL, frmL) | passIf(ROH, frmH) | 
                    RIE | passIf(RIL, toL) | passIf(RIH, toH))
    
    if getMasked(0b11_11_11_00, instruction) == 0b00_01_00:
        # read from memory
        toL, toH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MIS | MO | RIE | passIf(RIL, toL) | passIf(RIH, toH))

    if getMasked(0b11_11_11_00, instruction) == 0b00_01_01:
        # write memory
        frmL, frmH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MIS | MI | ROE | passIf(ROL, frmL) | passIf(ROH, frmH))
    
    if getMasked(0b11_11_11_00, instruction) == 0b00_01_10:
        # write memory register low
        frmL, frmH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MIS | WME | ROE | passIf(ROL, frmL) | passIf(ROH, frmH))

    if getMasked(0b11_11_11_00, instruction) == 0b00_01_11:
        # write memory register high
        frmL, frmH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MIS | WME | WMS | ROE | passIf(ROL, frmL) | passIf(ROH, frmH))

    if getMasked(0b11_11_11_00, instruction) == 0b00_10_00:
        # load immediate (next word in register)
        toL, toH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MO | RIE | passIf(RIL, toL) | passIf(RIH, toH))
        if step == s4:
            return IPA
    
    if getMasked(0b11_11_11_00, instruction) == 0b00_10_01:
        # zero register
        toL, toH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ZO | RIE | passIf(RIL, toL) | passIf(RIH, toH))

    if getMasked(0b11_11_00_00, instruction) == 0b01_00:
        # add a+b
        aL, aH = demux(getMasked(0b1100, instruction))
        bL, bH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s4:
            return (ROE | passIf(ROL, bL) | passIf(ROH, bH) | A2I)
        if step == s5:
            return (AO | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_00_00, instruction) == 0b01_01:
        # sub a-b
        aL, aH = demux(getMasked(0b1100, instruction))
        bL, bH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s4:
            return (ROE | passIf(ROL, bL) | passIf(ROH, bH) | A2I)
        if step == s5:
            return (AO | AOPL | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_00_00, instruction) == 0b01_10:
        # nand a nand b
        aL, aH = demux(getMasked(0b1100, instruction))
        bL, bH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s4:
            return (ROE | passIf(ROL, bL) | passIf(ROH, bH) | A2I)
        if step == s5:
            return (AO | AOPH | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_00_00, instruction) == 0b01_11:
        # xor a xor b
        aL, aH = demux(getMasked(0b1100, instruction))
        bL, bH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s4:
            return (ROE | passIf(ROL, bL) | passIf(ROH, bH) | A2I)
        if step == s5:
            return (AO | AOPL | AOPH | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_11_00, instruction) == 0b10_00_00:
        # addi a, 0xab
        aL, aH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (MO | A2I)
        if step == s4:
            return (IPA | ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s5:
            return (AO | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_11_00, instruction) == 0b10_00_01:
        # inc a
        aL, aH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ZO | A2I)
        if step == s4:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s5:
            return (CI | AO | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_11_00, instruction) == 0b10_00_10:
        # inc a
        aL, aH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ZO | A2I)
        if step == s4:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A1I)
        if step == s5:
            return (CI | AO | AOPL | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if getMasked(0b11_11_11_00, instruction) == 0b10_00_11:
        # neg a
        aL, aH = demux(getMasked(0b11, instruction))
        if step == s3:
            return (ZO | A1I)
        if step == s4:
            return (ROE | passIf(ROL, aL) | passIf(ROH, aH) | A2I)
        if step == s5:
            return (CI | AO | AOPL | RIE | passIf(RIL, aL) | passIf(RIH, aH))

    if instruction == 0b11_11_11_11:
        # halt
        if step == s3:
            return HLT

    return MRST

for instruction in range(256):
    for flags in range(16):
        for step in range(16):
            memory[(instruction << 8) | (flags << 4) | step] = getStep(instruction, flags, step)
    
with open('rom.img', 'w') as f:
    f.write("v2.0 raw\n")
    for i in range(256*256):
        code = hex(memory[i])[2:]
        f.write(f"{code}\n")
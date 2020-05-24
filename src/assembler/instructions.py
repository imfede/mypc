from .Instruction import Instruction 
from .lines import *

def getMasked(mask, value):
    assert mask != 0
    offset = 0
    while (mask >> offset) & 0b1 == 0:
        offset += 1
    return (value & mask) >> offset

def hasFlag(flags, flag):
    return (flags & flag) != 0

def passIf(value, check):
    return value if check else NOOP

def registerIn(register):
    return (RIE | passIf(RIL, register & 0b1) | passIf(RIH, (register & 0b10) >> 1))

def registerOut(register):
    return (ROE | passIf(ROL, register & 0b1) | passIf(ROH, (register & 0b10) >> 1))

instructions = [
    Instruction(
        "MV",
        2,
        0b00_00_00_00, 
        0b11_11_00_00, 
        [ lambda i: (registerOut(getMasked(0b11, i)) | registerIn(getMasked(0b1100, i))) ]
    ),
    Instruction(
        "MEMR",
        1,
        0b00_01_00_00,
        0b11_11_11_00,
        [ lambda i: (MIS | MO | registerIn(getMasked(0b11, i))) ]
    ),
    Instruction(
        "MEMW",
        1,
        0b00_01_01_00,
        0b11_11_11_00,
        [ lambda i: (MIS | MI | registerOut(getMasked(0b11, i))) ]
    ),
    Instruction(
        "MSRL",
        1,
        0b00_01_10_00,
        0b11_11_11_00,
        [ lambda i: (WME | registerOut(getMasked(0b11, i))) ]
    ),
    Instruction(
        "MSRH",
        1,
        0b00_01_11_00,
        0b11_11_11_00,
        [ lambda i: (WME | WMS | registerOut(getMasked(0b11, i))) ]
    ),
    Instruction(
        "LI",
        2,
        0b00_10_00_00,
        0b11_11_11_00,
        [
            lambda i: (MO | registerIn(getMasked(0b11, i))),
            lambda: (IPA)
        ]
    ),
    Instruction(
        "ZERO",
        1,
        0b00_10_01_00,
        0b11_11_11_00,
        [ lambda i: registerIn(getMasked(0b11, i)) ]
    ),
    Instruction(
        "RTWL",
        0,
        0b00_10_11_00,
        0b11_11_11_11,
        [ lambda: (MIS | MI | RETE) ]
    ),
    Instruction(
        "RTWH",
        0,
        0b00_10_11_01,
        0b11_11_11_11,
        [ lambda: (MIS | MI | RETE | RETS) ]
    ),
    Instruction(
        "RTRL",
        0,
        0b00_10_11_10,
        0b11_11_11_11,
        [ lambda: (MIS | MO | RETE | RETI) ]
    ),
    Instruction(
        "RTRH",
        0,
        0b00_10_11_11,
        0b11_11_11_11,
        [ lambda: (MIS | MO | RETE | RETI | RETS) ]
    ),
    Instruction(
        "ADD",
        2,
        0b01_00_00_00,
        0b11_11_00_00,
        [
            lambda i: (A1I | registerOut(getMasked(0b1100, i))),
            lambda i: (A2I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | registerIn(getMasked(0b1100, i)))
        ]
    ),
    Instruction(
        "SUB",
        2,
        0b01_01_00_00,
        0b11_11_00_00,
        [
            lambda i: (A1I | registerOut(getMasked(0b1100, i))),
            lambda i: (A2I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | AOPL | registerIn(getMasked(0b1100, i)))
        ]
    ),
    Instruction(
        "NAND",
        2,
        0b01_10_00_00,
        0b11_11_00_00,
        [
            lambda i: (A1I | registerOut(getMasked(0b1100, i))),
            lambda i: (A2I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | AOPH | registerIn(getMasked(0b1100, i)))
        ]
    ),
    Instruction(
        "XOR",
        2,
        0b01_11_00_00,
        0b11_11_00_00,
        [
            lambda i: (A1I | registerOut(getMasked(0b1100, i))),
            lambda i: (A2I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | AOPL | AOPH | registerIn(getMasked(0b1100, i)))
        ]
    ),
    Instruction(
        "ADDI",
        2,
        0b10_00_00_00,
        0b11_11_11_00,
        [
            lambda: (MO | A2I),
            lambda i: (IPA | A1I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | registerIn(getMasked(0b11, i)))
        ]
    ),
    Instruction(
        "INC",
        1,
        0b10_00_01_00,
        0b11_11_11_00,
        [
            lambda: (ONEO | A2I),
            lambda i: (A1I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | registerIn(getMasked(0b11, i)))
        ]
    ),
    Instruction(
        "DEC",
        1,
        0b10_00_10_00,
        0b11_11_11_00,
        [
            lambda: (ONEO | A2I),
            lambda i: (A1I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | AOPL | registerIn(getMasked(0b11, i)))
        ]
    ),
    Instruction(
        "NEG",
        1,
        0b10_00_11_00,
        0b11_11_11_00,
        [
            lambda: (A1I),
            lambda i: (A2I | registerOut(getMasked(0b11, i))),
            lambda i: (AO | CI | AOPL | registerIn(getMasked(0b11, i)))
        ]
    ),
    Instruction(
        "PJMP",
        2,
        0b11_00_00_00,
        0b11_11_11_11,
        [
            lambda: (MO | JMPE | JMPI | JMPS),
            lambda: (IPA),
            lambda: (MO | JMPE | JMPI),
            lambda: (IPA)
        ]
    ),
    Instruction(
        "JMP",
        0,
        0b11_00_00_01,
        0b11_11_11_11,
        [
            lambda: (JMPE | IPE),
            lambda: (JMPE | JMPS | IPE | IPS)
        ]
    ),
    Instruction(
        "JAL",
        0,
        0b11_00_00_10,
        0b11_11_11_11,
        [
            lambda: (IPE | IPO | RETE | RETI),
            lambda: (IPE | IPO | IPS | RETE | RETI | RETS),
            lambda: (JMPE | IPE),
            lambda: (JMPE | JMPS | IPE | IPS)
        ]
    ),
    Instruction(
        "RET",
        0,
        0b11_00_00_11,
        0b11_11_11_11,
        [
            lambda: (RETE | IPE),
            lambda: (RETE | RETS | IPE | IPS)
        ]
    ),
    Instruction(
        "JCR",
        1,
        0b11_00_01_00,
        0b11_11_11_11,
        [
            lambda i, f: (A1I | IPE | IPO) if hasFlag(f, CO) else (IPA | MRST),
            lambda: (MO | A2I),
            lambda: (AO | CI | IPE),
            lambda i, f: (MRST) if not hasFlag(f, CO) else (FFO | A2I) if hasFlag(f, A2G1) else (ONEO | A2I),
            lambda: (IPE | IPO | IPS | A1I),
            lambda: (AO | IPE | IPS)
        ]
    ),
    Instruction(
        "JZR",
        1,
        0b11_00_01_01,
        0b11_11_11_11,
        [
            lambda i, f: (A1I | IPE | IPO) if hasFlag(f, FZ) else (IPA | MRST),
            lambda: (MO | A2I),
            lambda: (AO | CI | IPE),
            lambda i, f: (MRST) if not hasFlag(f, CO) else (FFO | A2I) if hasFlag(f, A2G1) else (ONEO | A2I),
            lambda: (IPE | IPO | IPS | A1I),
            lambda: (AO | IPE | IPS)
        ]
    ),
    Instruction(
        "JNR",
        1,
        0b11_00_01_10,
        0b11_11_11_11,
        [
            lambda i, f: (A1I | IPE | IPO) if hasFlag(f, NEG) else (IPA | MRST),
            lambda: (MO | A2I),
            lambda: (AO | CI | IPE),
            lambda i, f: (MRST) if not hasFlag(f, CO) else (FFO | A2I) if hasFlag(f, A2G1) else (ONEO | A2I),
            lambda: (IPE | IPO | IPS | A1I),
            lambda: (AO | IPE | IPS)
        ]
    ),
    Instruction(
        "JLTR",
        1,
        0b11_00_01_11,
        0b11_11_11_11,
        [
            lambda i, f: (A1I | IPE | IPO) if hasFlag(f, A2G1) else (IPA | MRST),
            lambda: (MO | A2I),
            lambda: (AO | CI | IPE),
            lambda i, f: (MRST) if not hasFlag(f, CO) else (FFO | A2I) if hasFlag(f, A2G1) else (ONEO | A2I),
            lambda: (IPE | IPO | IPS | A1I),
            lambda: (AO | IPE | IPS)
        ]
    ),
    Instruction(
        "SPSL",
        1,
        0b11_11_00_00,
        0b11_11_11_00,
        [ lambda i: (SPE | SPI | registerOut(getMasked(0b11, i))) ]
    ),
    Instruction(
        "SPSH",
        1,
        0b11_11_01_00,
        0b11_11_11_00,
        [ lambda i: (SPE | SPI | SPS | registerOut(getMasked(0b11, i))) ]
    ),
    Instruction(
        "PUSH",
        0,
        0b11_11_10_00,
        0b11_11_11_11,
        [
            lambda: (SPE | A1I | WME),
            lambda: (ONEO | A2I),
            lambda: (AO | AOPL | SPE | SPI),
            lambda i, f: (NOOP) if hasFlag(f, A2G1) else (A2I),
            lambda: (SPE | SPS | A1I | WME | WMS),
            lambda: (AO | AOPL | SPE | SPI | SPS)
        ]
    ),
    Instruction(
        "PULL",
        0,
        0b11_11_10_01,
        0b11_11_11_11,
        [
            lambda: (SPE | A1I),
            lambda: (ONEO | A2I),
            lambda: (AO | SPE | SPI | WME),
            lambda i, f: (NOOP) if hasFlag(f, CO) else (A2I),
            lambda: (SPE | SPS | A1I),
            lambda: (AO | SPE | SPI | SPS | WME | WMS)
        ]
    ),
    Instruction(
        "PEEK",
        0,
        0b11_11_10_10,
        0b11_11_11_11,
        [
            lambda: (SPE | WME),
            lambda: (SPE | SPS | WME | WMS)
        ]
    ),
    Instruction(
        "SPOF",
        1,
        0b11_11_10_11,
        0b11_11_11_11,
        [
            lambda: (MO | A1I),
            lambda: (IPA | SPE | A2I),
            lambda: (AO | WME),
            lambda i, f: (ONEO | A2I) if hasFlag(f, CO) else (A2I),
            lambda: (SPE | SPS | A1I),
            lambda: (AO | WME | WMS)
        ]
    ),
    Instruction(
        "HLT",
        0,
        0b11_11_11_11,
        0b11_11_11_11,
        [
            lambda: (HLT)
        ]
    )
]
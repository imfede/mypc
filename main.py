#!/usr/bin/env python3

from src.assembler.assembler import main as assemble
from src.burner.burner import main as burn
from src.compiler.compiler import main as compile
from sys import argv

# ./main.py assemble|burn|compile <fname>

cmd = argv[1]

if cmd == "assemble":
    fname = argv[2]
    print(f"Assembling {fname}")
    assemble(fname)

elif cmd == "burn":
    print("Burning roms: rom01.img rom02.img")
    burn()

elif cmd == "compile":
    fname = argv[2]
    print(f"Compiling {fname}")
    compile(fname)
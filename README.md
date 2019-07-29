# What

This is the project for my personal computer.

- `schema.circ` is the logisim file containing the wiring
- `reference.txt` describe all the implemented instructions (it's the assembly reference)
- `src/assembler.py` will take a txt file containing assembly code (like `examples/fib.as`) and will output the sequence of bytes (in a file named `ram.img`) ready to be loaded into the system RAM
- `src/burner.py` will output `rom01.img` and `rom02.img` binary files ready to be loaded into the control ROMs.

# TODO

In no particular order:
- write a "compiler" capable of translating a simple language into assembly
# What

This is the project for my personal computer.

- `schema.circ` is the logisim file containing the wiring
- `reference.txt` describe all the implemented instructions (it's the assembly reference)
- `parser.py` will take a txt file containing assembly code (like `examples/fib.as`) and will output the sequence of bytes (in a file named `ram.img`) ready to be loaded into the system RAM
- `romburner.py` will output a `rom.img` binary file ready to be loaded into the control ROM.

# TODO

In no particular order:
- refactor the python files: they are horrible and the cursed child of cmd+c cmd+v
- write a "compiler" capable of translating a simple language into assembly
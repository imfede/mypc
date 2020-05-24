# What

This is the project for my personal computer.

- `schema.circ` is the logisim file containing the wiring
- `reference.txt` describe all the implemented instructions (it's the assembly reference)
- `src/burner.py` will output `rom01.img` and `rom02.img` binary files ready to be loaded into the control ROMs.

## Decoding roms

To generate run

```bash
./main.py burn
```

It will create two files `rom01.img` and `rom02.img` ready to be loded into the logisim roms.

## Assembler

The assembler takes as an input an assembly file and outputs a ram image ready to be loaded into the logisim ram.

```bash
python3 src/assembler/assembler.py examples/fib.as # will output ram.img
```

## Compiler

The compiler takes as an input a source file and outputs an assembly text file.

# TODO

In no particular order:
- write a "compiler" capable of translating a simple language into assembly
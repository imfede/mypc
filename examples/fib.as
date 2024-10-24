# Programm to calculate the Fibonacci sequence
# Start the sequence from 0x01 and 0x00
LI A, 0x01
LI B, 0x00

:loop
    ADD A, B
    # Use D as a temporary register to swap A and B
    MV D, A
    MV A, B 
    MV B, D
    JCR .halt
    PJMP :loop
    JMP

.halt
HLT
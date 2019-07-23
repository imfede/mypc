LI A, 0x01
LI B, 0x00

:loop
    ADD A, B
    MV D, A
    MV A, B
    MV B, D
    JCR .halt
    PJMP :loop
    JMP

.halt
HLT
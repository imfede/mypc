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

IPE  = 1 << 24
IPO  = 1 << 25
IPS  = 1 << 26
ONEO = 1 << 27
FFO  = 1 << 28
SPE  = 1 << 29
SPI  = 1 << 30
SPS  = 1 << 31

JMPI = 1 << 32
JMPE = 1 << 33
JMPS = 1 << 34
RETI = 1 << 35
RETE = 1 << 36
RETS = 1 << 37

# flags

FZ   = 1 << 0
CO   = 1 << 1
A2G1 = 1 << 2
NEG  = 1 << 3
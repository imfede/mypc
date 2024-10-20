#[allow(unused, clippy::upper_case_acronyms)]
pub enum ControlLine {
    /// Reset, same as the reset button.
    RST,
    /// Micro Reset, reset the step counter.
    MRST,
    /// Register Output Enable, put the content of the selected register (see ROH and ROL) on the bus.
    ROE,
    /// Register Output Low, select the low byte of the register to output.
    ROL,
    /// Register Output High, select the high byte of the register to output.
    ROH,
    /// Register Input Enable, put the content of the bus on the selected register (see RIL and RIH).
    RIE,
    /// Register Input Low, select the low byte of the register to input.
    RIL,
    /// Register Input High, select the high byte of the register to input.
    RIH,
    /// Halt, stop the clock.
    HLT,
    /// Memory In, put the content of the bus into the RAM.
    MI,
    /// Write Memory Enable, write the content of the bus into the selected RAM address register (see WMS and MIS).
    WME,
    /// Write Memory Select, set to 0 to select the RAM address register low, set to 1 to select the RAM address register high.
    WMS,
    /// Memory Input Select, set to 0 to select the select the RAM address from the Instruction Pointer register, set to 1 to select the RAM address from the RAM address register.
    MIS,
    /// Instruction Pointer Advance, advance the Instruction Pointer register by 1.
    IPA,
    /// Memory Output, put the content of the selected RAM address register (see WMS and MIS) on the bus.
    MO,
    /// Instruction Register Enable, put the content of the bus into the Instruction Register.
    IRE,
    /// ALU 1 Input, put the content of the bus into the ALU 1 register.
    A1I,
    /// ALU 2 Input, put the content of the bus into the ALU 2 register.
    A2I,
    /// Carry Input, set the 1 to perform an operation with carry.
    CI,
    /// ALU Operation Low, select the low bit of the ALU operation.
    AOPL,
    /// ALU Operation High, select the high bit of the ALU operation.
    AOPH,
    /// ALU Out, put the content of the ALU into the bus.
    AO,
    /// Instruction Pointer Enable, ???.
    IPE,
    /// Instruction Pointer Output, put the content of the Instruction Pointer register on the bus.
    IPO,
    /// Instruction Pointer Select, set to 0 to select the Instruction Pointer Low register, set to 1 to select the Instruction Pointer High register.
    IPS,
    /// One Out, output 0x01 on the bus.
    ONEO,
    /// FF Out, output 0xFF on the bus.
    FFO,
    /// Stack Pointer Enable, set to 1 to read or write the selectedStack Pointer register (depending on SPI and SPS).
    SPE,
    /// Stack Pointer In, put the content of the bus into the selected Stack Pointer register (see SPS).
    SPI,
    /// Stack Pointer Select, set to 0 to select the Stack Pointer Low register, set to 1 to select the Stack Pointer High register.
    SPS,
    /// Jump Pointer In, put the content of the bus into the selected Jump Pointer register (see JMPS).
    JMPI,
    /// Jump Pointer Enable, set to 1 to read or write the selected Jump Pointer register (depending on JMPI and JMPS).
    JMPE,
    /// Jump Pointer Select, set to 0 to select the Jump Pointer Low register, set to 1 to select the Jump Pointer High register.
    JMPS,
    /// Return register In, put the content of the bus into the selected Return register (see RETS).
    RETI,
    /// Return register Enable, set to 1 to read or write the selected Return register (depending on RETI and RETS).
    RETE,
    /// Return register Select, set to 0 to select the Return Low register, set to 1 to select the Return High register.
    RETS,
}

impl ControlLine {
    pub fn value(&self) -> u64 {
        match self {
            ControlLine::RST => 1 << 0,
            ControlLine::MRST => 1 << 1,
            ControlLine::ROE => 1 << 2,
            ControlLine::ROL => 1 << 3,
            ControlLine::ROH => 1 << 4,
            ControlLine::RIE => 1 << 5,
            ControlLine::RIL => 1 << 6,
            ControlLine::RIH => 1 << 7,
            ControlLine::HLT => 1 << 8,
            ControlLine::MI => 1 << 9,
            ControlLine::WME => 1 << 10,
            ControlLine::WMS => 1 << 11,
            ControlLine::MIS => 1 << 12,
            ControlLine::IPA => 1 << 13,
            ControlLine::MO => 1 << 14,
            ControlLine::IRE => 1 << 15,
            ControlLine::A1I => 1 << 16,
            ControlLine::A2I => 1 << 17,
            ControlLine::CI => 1 << 18,
            ControlLine::AOPL => 1 << 19,
            ControlLine::AOPH => 1 << 20,
            ControlLine::AO => 1 << 21,
            ControlLine::IPE => 1 << 24,
            ControlLine::IPO => 1 << 25,
            ControlLine::IPS => 1 << 26,
            ControlLine::ONEO => 1 << 27,
            ControlLine::FFO => 1 << 28,
            ControlLine::SPE => 1 << 29,
            ControlLine::SPI => 1 << 30,
            ControlLine::SPS => 1 << 31,
            ControlLine::JMPI => 1 << 32,
            ControlLine::JMPE => 1 << 33,
            ControlLine::JMPS => 1 << 34,
            ControlLine::RETI => 1 << 35,
            ControlLine::RETE => 1 << 36,
            ControlLine::RETS => 1 << 37,
        }
    }
}

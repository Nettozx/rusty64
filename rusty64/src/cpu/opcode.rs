enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        ANDI = 0b00_1100,
        ORI  = 0b00_1101,
        LUI  = 0b00_1111,
        MTC0 = 0b01_0000,
        BEQL = 0b01_0100,
        LW   = 0b10_0011,
    }
}
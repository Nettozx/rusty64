enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        ADDI  = 0b00_1000,
        ADDIU = 0b00_1001,
        ANDI  = 0b00_1100,
        ORI   = 0b00_1101,
        LUI   = 0b00_1111,
        MTC0  = 0b01_0000,
        BEQL  = 0b01_0100,
        LW    = 0b10_0011,
        SW    = 0b10_1011,
    }
}
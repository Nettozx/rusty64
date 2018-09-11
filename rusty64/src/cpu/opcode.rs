enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        SPECIAL  = 0b00_0000, //for instructions with special bit code
        ADDI     = 0b00_1000, //page 372
        ADDIU    = 0b00_1001, //page 373
        ANDI     = 0b00_1100, //page 376
        ORI      = 0b00_1101, //page 485
        LUI      = 0b00_1111, //page 456
        MTC0     = 0b01_0000, //page 474
        BEQ      = 0b00_0100, //page 385
        BNE      = 0b00_0101, //page 399
        BEQL     = 0b01_0100, //page 386
        BNEL     = 0b01_0101, //page 400

        LW       = 0b10_0011, //page 458
        SW       = 0b10_1011, //page 515
    }
}

enum_from_primitive! {
#[derive(Debug)]
pub enum SpecialOpcode {
        SRL   = 0b00_0010, //page 511
        JR    = 0b00_1000, //page 438
        MFLO  = 0b01_0010, //page 473
        MULTU = 0b01_1001, //page 481
        OR    = 0b10_0101, //page 484
    }
}
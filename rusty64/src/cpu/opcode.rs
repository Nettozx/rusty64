enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        SPECIAL  = 0b00_0000, //for instructions with special bit code
        REGIMM   = 0b00_0001, //for instructions with regimm bit code
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
        SLL   = 0b00_0000, //page 503, same code as NOP
        SRL   = 0b00_0010, //page 511
        SLLV  = 0b00_0100, //page 504
        SRLV  = 0b00_0110, //page 512
        JR    = 0b00_1000, //page 438
        MFHI  = 0b01_0000, //page 472
        MFLO  = 0b01_0010, //page 473
        MULTU = 0b01_1001, //page 481
        ADDU  = 0b10_0001, //page 374
        SUBU  = 0b10_0011, //page 514
        AND   = 0b10_0100, //page 375
        OR    = 0b10_0101, //page 484
        XOR   = 0b10_0110, //page 542
        SLTU  = 0b10_1011, //page 508
    }
}

enum_from_primitive! {
#[derive(Debug)]
pub enum RegImmOpcode {
        BGEZAL = 0b10001, //page 388
    }
}
enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        SPECIAL  = 0b00_0000, //page 81 - for instructions with special bit code
        REGIMM   = 0b00_0001, //page 79 - for instructions with regimm bit code
        COP0     = 0b01_0000, //page 83,407 - for instructions with COP0 bit code
        ADDI     = 0b00_1000, //page 372
        ADDIU    = 0b00_1001, //page 373
        ANDI     = 0b00_1100, //page 376
        ORI      = 0b00_1101, //page 485
        LUI      = 0b00_1111, //page 456
        //MTC0     = 0b01_0000, //page 474 TODO this is a sub instruction of COP0
        BEQ      = 0b00_0100, //page 385
        BNE      = 0b00_0101, //page 399
        BGTZ     = 0b00_0111, //page 391
        BGTZL    = 0b01_0111, //page 392
        BLEZ     = 0b00_0110, //page 393
        BLEZL    = 0b01_0110, //page 394
        XORI     = 0b00_1110, //page 543
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
        BGEZ      = 0b00001, //page 387
        BGEZAL    = 0b10001, //page 388
        BGEZALL   = 0b10011, //page 389
        BGEZL     = 0b00011, //page 390
        BLTZ      = 0b00000, //page 395
        BLTZAL    = 0b10000, //page 396
        BLTZALL   = 0b10010, //page 397
        BLTZL     = 0b00010, //page 398
    }
}

enum_from_primitive! {
#[derive(Debug)]
pub enum Cp0Opcode {
        MT = 0b00100,   //page 474 this is MTC0 when used with COP0
    }
}

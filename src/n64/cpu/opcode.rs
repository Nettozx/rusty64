enum_from_primitive! {
    #[derive(Debug)]
    pub enum Opcode {
        SPECIAL  = 0b00_0000, //page 81 - for instructions with special bit code
        REGIMM   = 0b00_0001, //page 79 - for instructions with regimm bit code
        //COP0     = 0b01_0000, //page 83,407 - for instructions with COP0 bit code
        //Section 3.1 CPU Instruction Formats - page 60
        //I-Type (Immediate)
        //op(31-26), rs(25-21), rt(20-16), immediate(15-0)
        //** ALU Immediate Instructions **
        ADDI     = 0b00_1000, //page 372
        ADDIU    = 0b00_1001, //page 373
        SLTI     = 0b00_1010, //page 506
        SLTIU    = 0b00_1011, //page 507
        ANDI     = 0b00_1100, //page 376
        ORI      = 0b00_1101, //page 485
        XORI     = 0b00_1110, //page 543
        LUI      = 0b00_1111, //page 456

        //Branching Instructions (Immediate format, diff variables)
        //op(31-26), rs(25-21), rt(20-16), offset(15-0)
        BEQ      = 0b00_0100, //page 385
        BNE      = 0b00_0101, //page 399
        BLEZ     = 0b00_0110, //page 393
        BGTZ     = 0b00_0111, //page 391
        BEQL     = 0b01_0100, //page 386
        BNEL     = 0b01_0101, //page 400
        BLEZL    = 0b01_0110, //page 394
        BGTZL    = 0b01_0111, //page 392

        //J-Type (Jump)
        //op(31-26), target(25-0)
        J        = 0b00_0010, //page 435
        JAL      = 0b00_0011, //page 436

        //Load/Store Instructions
        //op(31-26), base(25-21), rt(20-16), offset(15-0)
        LW       = 0b10_0011, //page 458
        LWU      = 0b10_0111, //page 467
        SW       = 0b10_1011, //page 515

        MTC0     = 0b01_0000, //page 474 TODO this is a sub instruction of COP0
    }
}

enum_from_primitive! {
#[derive(Debug)]
pub enum SpecialOpcode {
        //Section 3.1 CPU Instruction Formats - page 60
        //R-Type (Register) (aka Three-Operand Type Instructions)
        //op(31-26), rs(25-21), rt(20-16), rd(15-11), sa(10-6), funct(5-0)
        //** Three-Operand Type Instructions **
        ADD   = 0b10_0000, //page 371
        ADDU  = 0b10_0001, //page 374
        SUB   = 0b10_0010, //page 513
        SUBU  = 0b10_0011, //page 514
        SLT   = 0b10_1010, //page 505
        SLTU  = 0b10_1011, //page 508
        AND   = 0b10_0100, //page 375
        OR    = 0b10_0101, //page 484
        XOR   = 0b10_0110, //page 542
        NOR   = 0b10_0111, //page 483
        //** Shift Instructions **
        SLL   = 0b00_0000, //page 503, same code as NOP
        SRL   = 0b00_0010, //page 511
        //SRA
        SLLV  = 0b00_0100, //page 504
        SRLV  = 0b00_0110, //page 512
        //SRAV
        //** Multiply/Divide Instructions **
        //MULT
        MULTU = 0b01_1001, //page 481
        //DIV
        //DIVU
        MFHI  = 0b01_0000, //page 472
        MFLO  = 0b01_0010, //page 473
        //MTHI
        //MTLO
        //** Jump Instructions **
        JR    = 0b00_1000, //page 438
        //JALR = 0b00_1001, //page 437
    }
}

enum_from_primitive! {
#[derive(Debug)]
pub enum RegImmOpcode {
        //Branching Instructions (Immediate format, diff variables)
        //REGIMM(31-26), rs(25-21), sub(20-16), offset(15-0)
        //REGIMM = 000001
        //sub = instruction code
        BLTZ      = 0b00000, //page 395
        BGEZ      = 0b00001, //page 387
        BLTZAL    = 0b10000, //page 396
        BGEZAL    = 0b10001, //page 388
        BLTZL     = 0b00010, //page 398
        BGEZL     = 0b00011, //page 390
        BLTZALL   = 0b10010, //page 397
        BGEZALL   = 0b10011, //page 389
    }
}

//enum_from_primitive! {
//#[derive(Debug)]
//pub enum Cp0Opcode {
//        MT = 0b00100,   //page 474 this is MTC0 when used with COP0
//    }
//}

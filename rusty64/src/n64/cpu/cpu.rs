use super::super::interconnect::*;
use super::cp0;
use super::opcode::Opcode::*;
use super::opcode::SpecialOpcode::*;
use super::opcode::RegImmOpcode::*;
use super::instruction::Instruction;

use std::fmt;

const NUM_GPR: usize = 32; //number of general purpose registers

enum SignExtendResult {
    Yes,
    No
}

enum WriteLink {
    Yes,
    No
}

pub struct Cpu {
    //section 1.4.2 CPU Registers on datasheet
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR],

    reg_pc: u64,

    reg_hi: u64,
    reg_lo: u64,

    reg_llbit: bool, //TODO: Enum type

    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0,

    delay_slot_pc: Option<u64>
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],
            //power on reset value, section 9.2.1 in datasheet & page 153
            reg_pc: 0xffff_ffff_bfc0_0000, //TODO move to const
            reg_hi: 0,
            reg_lo: 0,
            reg_llbit: false,
            reg_fcr0: 0,
            reg_fcr31: 0,
            cp0: cp0::Cp0::default(),
            delay_slot_pc: None,
        }
    }

    pub fn current_pc_virt(&self) -> u64 {
        self.delay_slot_pc.unwrap_or(self.reg_pc)
    }

    pub fn current_pc_phys(&self) -> u64 {
        self.virt_addr_to_phys_addr(self.current_pc_virt())
    }

    pub fn will_execute_from_delay_slot(&self) -> bool {
        self.delay_slot_pc.is_some()
    }

    pub fn step(&mut self, interconnect: &mut Interconnect) {
        if let Some(pc) = self.delay_slot_pc {
            let instr = self.read_instruction(interconnect,pc);
            self.execute_instruction(interconnect, instr);
            //clear delay slot after using
            self.delay_slot_pc = None;
        } else {
            let instr = self.read_instruction(interconnect,self.reg_pc);
            //increment prog counter
            self.reg_pc += 4;
            //execute the instruction
            self.execute_instruction(interconnect, instr);
        }
    }

    fn read_instruction(&self, interconnect: &mut Interconnect, addr: u64) -> Instruction {
        Instruction(self.read_word(interconnect,addr))
    }

    fn execute_instruction(&mut self, interconnect: &mut Interconnect, instr: Instruction) {
        //section 16.6 of datasheet
        match instr.opcode() {
            SPECIAL => match instr.special_op() {
                SLL => {
                    //Shift Left Logical - page 503
                    //same as NOP if sa==0
                    self.reg_instr(instr, |_, rt, sa| {
                        rt << sa
                    })
                }
                SRL => {
                    //Shift Right Logical - page 511
                    self.reg_instr(instr, |_, rt, sa| {
                        let rt = rt as u32;
                        (rt >> sa) as u64
                    })
                }
                SLLV => {
                    //Shift Left Logical Variable - page 504
                    self.reg_instr(instr,|rs,rt, _| {
                        let shift = rs & 0b11111;
                        rt << shift
                    })
                }
                SRLV => {
                    //Shift Right Logical Variable - page 512
                    self.reg_instr(instr, |rs,rt, _| {
                        let rs = rs as u32;
                        let rt = rt as u32;
                        let shift = rs & 0b11111;
                        (rt >> shift) as u64
                    })
                }
                JR => {
                    //Jump Register - page 438
                    //get the old program counter cause it needs delay slot
                    let delay_slot_pc = self.reg_pc;
                    //Update PC before executing delay slot instruction
                    self.reg_pc = self.read_reg_gpr(instr.rs());
                    self.delay_slot_pc = Some(delay_slot_pc);
                }
                MFHI => {
                    //Move From HI - page 472
                    let value = self.reg_hi;
                    self.write_reg_gpr(instr.rd() as usize, value);
                }
                MFLO => {
                    //Move From LO - page 473
                    let value = self.reg_lo;
                    self.write_reg_gpr(instr.rd() as usize, value);
                }
                MULTU => {
                    //Multiply Unsigned - page 481
                    //TODO undefined if last 2 instr were MFHI or MFLO
                    let rs = self.read_reg_gpr(instr.rs()) as u32;
                    let rt = self.read_reg_gpr(instr.rt()) as u32;

                    //sign extend product
                    let res = (rs as u64) * (rt as u64);
                    self.reg_lo = (res as i32) as u64;
                    self.reg_hi = ((res >> 32) as i32) as u64;
                }
                ADDU => {
                    //Add Unsigned page - 374
                    self.reg_instr(instr, |rs,rt, _| { rs.wrapping_add(rt) })
                }
                SUBU => {
                    //Subtract Unsigned - page 514
                    self.reg_instr(instr, |rs, rt, _| { rs.wrapping_sub(rt) })
                }
                AND => {
                    //And - page 375
                    self.reg_instr(instr, |rs, rt, _| { rs & rt })
                }
                OR => {
                    //Or - page 484
                    self.reg_instr(instr, |rs, rt, _| { rs | rt })
                }
                XOR => {
                    //Exclusive Or - page 542
                    self.reg_instr(instr, |rs, rt, _| { rs ^ rt })
                }
                SLTU => {
                    //Set On Less Than Unsigned - page 508, ignored subtraction made no sense
                    self.reg_instr(instr, |rs, rt, _|
                        { if rs < rt { 1 } else { 0 } })
                }
            },
            REGIMM => match instr.reg_imm_op() {
                BGEZAL => {
                    //Branch On Greater Than Or Equal To Zero And Link - page 388
                    self.branch(instr, WriteLink::Yes, |rs, _| (rs as i64) >= 0);
                }
            },
            ADDI => {
                //Add Immediate - page 372
                self.imm_instr(instr, SignExtendResult::Yes,
                               |rs, _, imm_sign_extended| {
                    rs.wrapping_add(imm_sign_extended)
                    //TODO just doing wrapping add to ignore error
                })
            }
            ADDIU => {
                //Add Immediate Unsigned - page 373
                self.imm_instr(instr, SignExtendResult::Yes,
                               |rs, _, imm_sign_extended| {
                    rs.wrapping_add(imm_sign_extended)
                })
            }
            ANDI => {
                //And Immediate - page 376
                self.imm_instr(instr, SignExtendResult::No,
                               |rs, imm, _| { rs & imm })
            }
            ORI => {
                //Or Immediate - page 485
                self.imm_instr(instr, SignExtendResult::No,
                               |rs, imm, _| { rs | imm })
            }
            LUI => {
                //Load Upper Immediate - page 456
                self.imm_instr(instr, SignExtendResult::Yes,
                               |_, imm, _| { imm << 16 })
            }
            MTC0 => {
                //Move To System Control Coprocessor - page 474
                let data = self.read_reg_gpr(instr.rt());
                self.cp0.write_reg(instr.rd(), data);
            }
            BEQ => {
                //Branch On Equal - page 385
                self.branch(instr, WriteLink::No, |rs, rt| rs == rt);
            }
            BNE => {
                //Branch On Not Equal - page 399
                self.branch(instr, WriteLink::No, |rs, rt| rs != rt);
            }
            BEQL => {
                //Branch On Equal Likely - page 386
                //BEQZL is the same but with zero filled in already
                self.branch_likely(instr, |rs, rt| rs == rt);
            }
            BNEL => {
                //Branch On Not Equal Likely - page 400
                //BNEZL is the same but with zero filled in already
                self.branch_likely(instr, |rs, rt| rs != rt);
            }
            LW => {
                //Load Word - page 458
                let base = instr.rs();
                //sign extend for upper 32 bits
                let sign_extended_offset = instr.imm_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = (self.read_word(interconnect, virt_addr) as i32) as u64;
                self.write_reg_gpr(instr.rt(), mem);
            }
            SW => {
                //Store Word - page 515
                let base = instr.rs();
                //sign extend for upper 32 bits
                let sign_extended_offset = instr.imm_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_reg_gpr(instr.rt()) as u32;
                self.write_word(interconnect, virt_addr, mem);
            }
        }
    }

    fn imm_instr<F>(&mut self, instr: Instruction, sign_extend_result: SignExtendResult, f: F)
        where F: FnOnce(u64, u64, u64) -> u64 {
        let rs = self.read_reg_gpr(instr.rs());
        let imm = instr.imm() as u64;
        let imm_sign_extended = instr.imm_sign_extended();
        let value = f(rs, imm, imm_sign_extended);
        let sign_extended_value = (value as i32) as u64;
        let value = match sign_extend_result {
            SignExtendResult::Yes => sign_extended_value,
            _ => value
        };
        self.write_reg_gpr(instr.rt(), value);
    }

    fn reg_instr<F>(&mut self, instr: Instruction, f: F) where F: FnOnce(u64, u64, u32) -> u64 {
        //formatting based on section 1.4.3 in datasheet
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let sa = instr.sa();
        let value = f(rs, rt, sa);
        let sign_extended_value = (value as i32) as u64;
        self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
    }

    //branch lambda expression
    fn branch<F>(&mut self, instr: Instruction, write_link: WriteLink, f: F) -> bool
        where F: FnOnce(u64, u64) -> bool {
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let is_taken = f(rs, rt);

        //get the old program counter cause it needs delay slot
        let delay_slot_pc = self.reg_pc;
        //for regimm instructions
        if let WriteLink::Yes = write_link {
            let link_address = delay_slot_pc + 4;
            self.write_reg_gpr(31, link_address);
        }

        if is_taken {
            let sign_extended_offset = instr.offset_sign_extended() << 2;
            //Update PC before executing delay slot instruction
            self.reg_pc = self.reg_pc.wrapping_add(sign_extended_offset);

            self.delay_slot_pc = Some(delay_slot_pc);
        }
        is_taken
    }

    //branch likely lambda expression
    fn branch_likely<F>(&mut self, instr: Instruction, f: F)
        where F: FnOnce(u64, u64) -> bool {
        if !self.branch(instr, WriteLink::No, f) {
            //skip delay slot when branch not taken
            self.reg_pc = self.reg_pc.wrapping_add(4);
        }
    }

    fn read_word(&self, interconnect: &mut Interconnect, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.read_word(phys_addr as u32)
    }

    fn write_word(&mut self, interconnect: &mut Interconnect, virt_addr: u64, value: u32) {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        interconnect.write_word(phys_addr as u32, value)
    }

    fn virt_addr_to_phys_addr(&self, virt_addr: u64) -> u64 {
        //page 134 of datasheet table 5-3
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        if addr_bit_values == 0b101 {
            // kseg1
            virt_addr - 0xffff_ffff_a000_0000
        } else {
            //TODO
            panic!("Unrecognized virtual address: {:#x}", virt_addr);
        }
    }

    fn write_reg_gpr(&mut self, index: usize, value: u64) {
        if index != 0 {
            self.reg_gpr[index] = value;
        }
    }

    fn read_reg_gpr(&self, index: usize) -> u64 {
        match index {
            0 => 0,
            _ => self.reg_gpr[index]
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPR] = [
            "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
            "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
            "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
            "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ];

        write!(f, "\nCPU General Purpose Registers:\n")?;
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                writeln!(f, "")?;
            }
            write!(f, "{reg_name}/gpr{num:02}: {value:#018X} ",
                   num = reg_num,
                   reg_name = REG_NAMES[reg_num],
                   value = self.reg_gpr[reg_num],
            )?;
        }

        write!(f, "\n\nCPU Floating Point Registers:\n")?;
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                writeln!(f, "")?;
            }
            write!(f,
                   "fpr{num:02}: {value:21} ",
                   num = reg_num,
                   value = self.reg_fpr[reg_num],
            )?;
        }

        writeln!(f, "\n\nCPU Special Registers:\n")?;
        writeln!(f,
                 "\
            reg_pc: {:#018X}\n\
            reg_hi: {:#018X}\n\
            reg_lo: {:#018X}\n\
            reg_llbit: {}\n\
            reg_fcr0: {:#010X}\n\
            reg_fcr31: {:#010X}\n\
            ",
                 self.reg_pc,
                 self.reg_hi,
                 self.reg_lo,
                 self.reg_llbit,
                 self.reg_fcr0,
                 self.reg_fcr31
        )?;

        writeln!(f, "{:#?}", self.cp0)
        //writeln!(f, "{:#?}", self.interconnect)
    }
}
use super::super::interconnect;
use super::cp0;
use super::opcode::Opcode::*;
use super::opcode::SpecialOpcode::*;
use super::instruction::Instruction;

use std::fmt;

const NUM_GPR: usize = 32; //number of general purpose registers

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

    interconnect: interconnect::Interconnect
}

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
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
            interconnect,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instr = self.read_instruction(self.reg_pc);

        println!("reg_pc: {:#018X}: {:?}", self.reg_pc, instr);

        //increment prog counter
        self.reg_pc += 4;
        //execute the instruction
        self.execute_instruction(instr);
    }

    fn read_instruction(&self, addr: u64) -> Instruction {
        Instruction(self.read_word(addr))
    }

    fn execute_instruction(&mut self, instr: Instruction) {
        //section 16.6 of datasheet
        match instr.opcode() {
            SPECIAL => match instr.special_op() {
                SRL => {
                    //SRL page 511
                    let value = self.read_reg_gpr(instr.rt()) >> instr.sa();
                    let sign_extended_value = (value as i32) as u64;
                    self.write_reg_gpr(instr.rd() as usize, sign_extended_value);
                },
                JR => {
                    //JR page 438
                    //get the old program counter cause it needs delay slot
                    let delay_slot_pc = self.reg_pc;

                    //Update PC before executing delay slot instruction
                    self.reg_pc = self.read_reg_gpr(instr.rs());

                    let delay_slot_instr = self.read_instruction(delay_slot_pc);
                    self.execute_instruction(delay_slot_instr);
                },
                MFLO => {
                    //MFLO page 473
                    let value = self.reg_lo;
                    self.write_reg_gpr(instr.rd() as usize, value);
                },
                MULTU => {
                    //MULTU page 481
                    //TODO undefined if last 2 instr were MFHI or MFLO
                    let rs = instr.rs() as u32;
                    let rt = instr.rt() as u32;
                    //sign extend product
                    let res = ((rs * rt) as i32) as u64;
                    self.reg_lo = (res as i32) as u64;
                    self.reg_hi = ((res >> 32) as i32) as u64;
                },
                OR => {
                    let value = self.read_reg_gpr(instr.rs()) |
                        self.read_reg_gpr(instr.rt());
                    self.write_reg_gpr(instr.rd() as usize, value);
                }
            },
            ADDI => {
                //ADDI page 372
                let  rs_positive = (self.read_reg_gpr(instr.rs()) >> 31) & 1 == 0;
                let imm_positive = (instr.imm_sign_extended() >> 31) & 1 == 0;
                let res = self.read_reg_gpr(instr.rs()).wrapping_add(instr.imm_sign_extended());
                let res_positive = (res >> 31 & 1) == 0;
                match (rs_positive, imm_positive, res_positive) {
                    (true, true, false) => {
                        panic!("Integer overflow exception not implemented! (p+p=n) {:016X} + \
                        {:016X} != {:016X}", instr.rs(), instr.imm_sign_extended(), res)
                    },
                    (false, false, true) => {
                        panic!("Integer overflow exception not implemented! (n+n=p) {:016X} + \
                        {:016X} != {:016X}", instr.rs(), instr.imm_sign_extended(), res)
                    },
                    _ => {}
                }

                //let res = self.read_reg_gpr(instr.rs()) + instr.imm_sign_extended();
                self.write_reg_gpr(instr.rt(), res)
            },
            ADDIU => {
                //ADDIU page 373
                //the same as ADDI but it cannot overflow
                let res = self.read_reg_gpr(instr.rs())
                    .wrapping_add(instr.imm_sign_extended());
                self.write_reg_gpr(instr.rt(), res)
            },
            ANDI => {
                //ANDI page 376
                let res = self.read_reg_gpr(instr.rs()) & (instr.imm() as u64);
                self.write_reg_gpr(instr.rt(), res);
            },
            ORI => {
                //ORI page 485
                let res = self.read_reg_gpr(instr.rs()) | (instr.imm() as u64);
                self.write_reg_gpr(instr.rt(), res);
            },
            LUI => {
                //LUI page 456
                //sign extend for upper 32 bits
                let value = ((instr.imm() << 16) as i32) as u64;
                self.write_reg_gpr(instr.rt(), value );
            },
            MTC0 => {
                //MTC0 page 474
                let data = self.read_reg_gpr(instr.rt());
                self.cp0.write_reg(instr.rd(), data);
            },
            BEQ => {
                self.branch(instr, |rs, rt| rs == rt);
            },
            BNE => {
                self.branch(instr, |rs, rt| rs != rt);
            }
            BEQL => {
                //BEQL, BEQZL is the same but with zero filled in already - page 386
                self.branch_likely(instr, |rs, rt| rs == rt);
            },
            BNEL => {
                //BNEL, BNEZL is the same but with zero filled in already - page 400
                self.branch_likely(instr, |rs, rt| rs != rt);
            },
            LW => {
                //LW page 458
                let base = instr.rs();
                //sign extend for upper 32 bits
                let sign_extended_offset = instr.imm_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = (self.read_word(virt_addr) as i32) as u64;
                self.write_reg_gpr(instr.rt(), mem);
            },
            SW => {
                //SW page 515
                let base = instr.rs();
                //sign extend for upper 32 bits
                let sign_extended_offset = instr.imm_sign_extended();
                let virt_addr =
                    self.read_reg_gpr(base).wrapping_add(sign_extended_offset);
                let mem = self.read_reg_gpr(instr.rt()) as u32;
                self.write_word(virt_addr, mem);
                let mem = (self.read_word(virt_addr) as i32) as u64;
            }
        }
    }

    //branch lambda expression
    fn branch<F>(&mut self, instr: Instruction, f: F) -> bool
        where F: FnOnce(u64, u64) -> bool {
        let rs = self.read_reg_gpr(instr.rs());
        let rt = self.read_reg_gpr(instr.rt());
        let is_taken = f(rs, rt);

        if is_taken {
            //get the old program counter cause it needs delay slot
            let delay_slot_pc = self.reg_pc;

            let sign_extended_offset = instr.offset_sign_extended() << 2;
            //Update PC before executing delay slot instruction
            self.reg_pc = self.reg_pc.wrapping_add(sign_extended_offset);
            //TODO make this safer cause it can stack overflow
            let delay_slot_instr = self.read_instruction(delay_slot_pc);
            self.execute_instruction(delay_slot_instr);
        }

        is_taken
    }

    //branch likely lambda expression
    fn branch_likely<F>(&mut self, instr: Instruction, f: F)
        where F: FnOnce(u64, u64) -> bool {
        if !self.branch(instr, f) {
            //skip delay slot when branch not taken
            self.reg_pc = self.reg_pc.wrapping_add(4);
        }
    }

    fn read_word(&self, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.read_word(phys_addr as u32)
    }

    fn write_word(&mut self, virt_addr: u64, value: u32) {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.write_word(phys_addr as u32, value)
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

        writeln!(f, "{:#?}", self.cp0)?;
        writeln!(f, "{:#?}", self.interconnect)
    }
}
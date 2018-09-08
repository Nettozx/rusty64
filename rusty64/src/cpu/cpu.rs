use super::super::interconnect;
use super::cp0;
use super::opcode::Opcode::*;
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

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPR] = [
            "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
            "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
            "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
            "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ];

        write!(f, "\nCPU General Purpose Registers:")?;
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

        write!(f, "\n\nCPU Floating Point Registers:")?;
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

        writeln!(f, "\n\nCPU Special Registers:")?;
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

impl Cpu {
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu {
        Cpu {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],
            reg_pc: 0,
            reg_hi: 0,
            reg_lo: 0,
            reg_llbit: false,
            reg_fcr0: 0,
            reg_fcr31: 0,
            cp0: cp0::Cp0::default(),
            interconnect,
        }
    }
    pub fn power_on_reset(&mut self) {
        self.cp0.power_on_reset();
        //read pages 134-136 in datasheet and look at memory map txt file
        self.reg_pc = 0xffff_ffff_bfc0_0000;
    }
    pub fn run(&mut self) {
        loop {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self) {
        let instr = Instruction(self.read_word(self.reg_pc));

        //section 16.6 of datasheet
        match instr.opcode() {
            ANDI => {
                //ANDI page 376
                let res = self.read_reg_gpr(instr.rs() as usize) & (instr.imm() as u64);
                self.write_reg_gpr(instr.rt() as usize, res);
            },
            ORI => {
                //ORI page 485
                let res = self.read_reg_gpr(instr.rs() as usize) | (instr.imm() as u64);
                self.write_reg_gpr(instr.rt() as usize, res);
            },
            LUI => {
                //LUI page 456
                //sign extend for upper 32 bits
                let value = ((instr.imm() << 16) as i32) as u64;
                self.write_reg_gpr(instr.rt() as usize, value );
            },
            MTC0 => {
                //MTC0 page 474
                let data = self.read_reg_gpr(instr.rt() as usize);
                self.cp0.write_reg(instr.rd(), data);
            },
            BEQL => {
                //BEQL, BEQZL is the same but with zero filled in already
                if self.read_reg_gpr(instr.rs() as usize) ==
                    self.read_reg_gpr(instr.rt() as usize){
                    let sign_extended_offset = ((instr.offset() as i16) as u64).wrapping_shl(2);
                    self.reg_pc = self.reg_pc.wrapping_add(sign_extended_offset);
                    //TODO make this safer cause it can stack overflow
                    self.run_instruction();
                }

            },
            LW => {
                //LW page 458
                let base = instr.rs();
                //sign extend for upper 32 bits
                let sign_extended_offset = (instr.offset() as i16) as u64;
                let virt_addr =
                    self.read_reg_gpr(base as usize).wrapping_add(sign_extended_offset);
                let mem = (self.read_word(virt_addr) as i32) as u64;
                self.write_reg_gpr(instr.rt() as usize, mem);
            }
        }

        self.reg_pc += 4;
    }

    fn read_word(&self, virt_addr: u64) -> u32 {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.read_word(phys_addr as u32)
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


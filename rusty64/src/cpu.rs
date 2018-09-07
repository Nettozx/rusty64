use super::interconnect;

const NUM_GPR: usize = 32; //number of general purpose registers

#[derive(Debug)]
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

    cp0: Cp0,

    interconnect: interconnect::Interconnect
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
            cp0: Cp0::default(),
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
        let instruction = self.read_word(self.reg_pc);

        //TODO check endian
        let opcode = (instruction >> 26) & 0b111111;
        //section 16.6 of datasheet
        match opcode {
            0b00_1111 => {
                //LUI page 456
                println!("LUI!");
                let imm = instruction & 0xffff;
                let rt = (instruction >> 16) & 0b11111;
                self.write_reg_gpr(rt as usize, (imm << 16) as u64 );
            },
            _ => panic!("Unrecognized instruction: {:#x}", instruction)
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
}

#[derive(Debug)]
enum RegConfigEp { //page153 on datasheet
    D,
    DxxDxx,
    RFU
}

impl Default for RegConfigEp {
    fn default() -> RegConfigEp {
        RegConfigEp::D
    }
}

#[derive(Debug)]
enum RegConfigBe { //section 5.4.6
LittleEndian,
    BigEndian
}

impl Default for RegConfigBe {
    fn default() -> RegConfigBe {
        RegConfigBe::BigEndian
    }
}

#[derive(Default, Debug)]
struct RegConfig {
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe
}

impl RegConfig {
    fn power_on_reset(&mut self) {
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;
    }
}

#[derive(Default, Debug)]
struct Cp0 {
    //page 46 on datasheet & chapter 5
    reg_config: RegConfig
}

impl Cp0 {
    fn power_on_reset(&mut self) {
        //section 9.2.1 in datasheet & page 153
        self.reg_config.power_on_reset();
    }
}
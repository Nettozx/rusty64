#[derive(Default)]
pub struct AudioInterface {
    dram_addr: u32,
    length: u32,
    //TODO check if I need all of these
//    reg_dram_addr: u32,
//    reg_len: u32,
//    reg_control: u32,
//    reg_status: u32,
//    reg_dacrate: u32,
//    reg_bitrate: u32,
}

impl AudioInterface {
    pub fn read_dram_addr_reg(&self) -> u32 {
        self.dram_addr
    }

    pub fn write_dram_addr_reg(&mut self, value: u32) {
        self.dram_addr = value & 0x00ff_ffff;
    }

    pub fn read_len_reg(&self) -> u32 {
        self.length
    }

    pub fn write_len_reg(&mut self, value: u32) {
        self.length = value & 0x0003_fff8; //ignore bottom 3 bits according to v2
    }
}
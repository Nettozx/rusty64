#[derive(Default)]
pub struct SerialInterface {
    //TODO check if I need all of these
//    reg_dram_addr: u32,
//    reg_pif_addr_rd64b: u32,
//    reg_pif_addr_wr64b: u32,
//    reg_status: u32,
}

impl SerialInterface {
    pub fn read_status_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        unimplemented!("Writes to SI status reg not yet implemented")
    }
}
pub struct Rdp {
    //TODO figure out if I need all of these
    //reg_start: u32,
    //reg_end: u32,
    //reg_current: u32,
    //reg_status: u32,
    //reg_clock: u32,
    //reg_bufbusy: u32,
    //reg_pipebusy: u32,
    //reg_tmem: u32,

    //reg_tbist: u32,
    //reg_test_mode: u32,
    //reg_buftest_addr: u32,
    //reg_buftest_data: u32,
}

impl Rdp {
    pub fn read_status_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        panic!("Write to RDP Status Reg: {:#?}", value);
    }
}
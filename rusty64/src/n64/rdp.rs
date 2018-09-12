pub struct Rdp;

impl Rdp {
    pub fn read_status_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        panic!("Write to RDP Status Reg: {:#?}", value);
    }
}
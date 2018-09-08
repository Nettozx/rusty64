#[derive(Default, Debug)]
pub struct Rsp;

impl Rsp {
    pub fn read_status_reg(&self) -> u32 {
        1//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        panic!("Writes to RSP status reg not yet supported: {:#018X}", value);
    }
}
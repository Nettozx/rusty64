#[derive(Default)]
pub struct SerialInterface();

impl SerialInterface {
    pub fn read_status_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        unimplemented!("Writes to SI status reg not yet implemented")
    }
}
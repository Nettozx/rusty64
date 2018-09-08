#[derive(Default)]
pub struct PeripheralInterface;

impl PeripheralInterface {
    pub fn read_status_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_status_reg(&mut self, value: u32) {
        if (value & (1 << 0)) != 0 {
            println!("WARNING: PI reset controller bit not yet implemented!");
        }

        if (value & (1 << 1)) != 0 {
            //TODO MI_INTR_REG is affected
            println!("WARNING: PI clear intr bit not yet implemented!");
        }
    }
}
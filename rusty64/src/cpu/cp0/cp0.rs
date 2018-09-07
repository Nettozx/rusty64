use super::reg_config;
use super::reg_status;

#[derive(Default, Debug)]
pub struct Cp0 {
    //page 46 on datasheet & chapter 5
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus
}

impl Cp0 {
    pub fn power_on_reset(&mut self) {
        //section 9.2.1 in datasheet & page 153
        self.reg_config.power_on_reset();
    }

    pub fn write_reg(&mut self, index: u32, data: u64) {
        match index {
            12 => { self.write_status_reg(data); }
            _ => panic!("Unrecognized Cp0 reg: {:#?}, {:#?}", index, data)
        }
    }

    fn write_status_reg(&mut self, data: u64) {
        // Status reg - section 6.3.5 in datasheet
        self.reg_status.write(data as u32);
    }
}
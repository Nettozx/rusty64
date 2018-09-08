use super::reg_status;
use super::reg_config;

#[derive(Default, Debug)]
pub struct Cp0 {
    //page 46 on datasheet & chapter 5
    reg_status: reg_status::RegStatus,
    reg_config: reg_config::RegConfig

}

impl Cp0 {
    pub fn write_reg(&mut self, index: u32, data: u64) {
        match index {
            // Status reg - section 6.3.5 in datasheet
            12 => { self.reg_status = (data as u32).into(); },
            // Config reg - section 5.4.6 in datasheet
            16 => { self.reg_config = (data as u32).into(); }
            _ => panic!("Unrecognized Cp0 reg: {}, {:#x}", index, data)
        }
    }
}
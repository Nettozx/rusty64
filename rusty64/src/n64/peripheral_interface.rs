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

    pub fn read_bsd_dom1_lat_reg(&self) -> u32 {
        0//not actually needed for now,
        // only has problems in few roms that check hardware timing
    }

    pub fn write_bsd_dom1_lat_reg(&mut self, value: u32) {
        //not needed for now
        println!("PI_BSD_DOM1_LAT_REG_WRITTEN: {:#018X}", value);
    }

    pub fn read_bsd_dom1_pwd_reg(&self) -> u32 {
        0//not actually needed for now,
        // only has problems in few roms that check hardware timing
    }

    pub fn write_bsd_dom1_pwd_reg(&mut self, value: u32) {
        //not needed for now
        println!("PI_BSD_DOM1_PWD_REG_WRITTEN: {:#018X}", value);
    }

    pub fn read_bsd_dom1_pgs_reg(&self) -> u32 {
        0//not actually needed for now,
        // only has problems in few roms that check hardware timing
    }

    pub fn write_bsd_dom1_pgs_reg(&mut self, value: u32) {
        //not needed for now
        println!("PI_BSD_DOM1_PGS_REG_WRITTEN: {:#018X}", value);
    }

    pub fn read_bsd_dom1_rls_reg(&self) -> u32 {
        0//not actually needed for now,
        // only has problems in few roms that check hardware timing
    }

    pub fn write_bsd_dom1_rls_reg(&mut self, value: u32) {
        //not needed for now
        println!("PI_BSD_DOM1_RLS_REG_WRITTEN: {:#018X}", value);
    }
}
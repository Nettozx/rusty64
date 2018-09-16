#[derive(Default)]
pub struct VideoInterface {
    interrupt_half_line: u32,

    active_video_start: u32,
    active_video_end: u32,
//TODO check if I need all of these
//    reg_status: u32,
//    reg_origin: u32,
//    reg_width: u32,
//    reg_intr: u32,
//    reg_current: u32,
//    reg_burst: u32,
//    reg_v_sync: u32,
//    reg_h_sync: u32,
//    reg_leap: u32,
//    reg_h_start: u32,
//    reg_v_start: u32,
//    reg_v_burst: u32,
//    reg_x_scale: u32,
//    reg_y_scale: u32,
}

impl VideoInterface {
    pub fn read_intr_reg(&self) -> u32 {
        self.interrupt_half_line
    }

    pub fn write_intr_reg(&mut self, value: u32) {
        self.interrupt_half_line = value & 0x0000_03ff;
    }

    pub fn read_current_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_current_reg(&mut self, value: u32) {
        //TODO clear interrupt
    }

    pub fn read_h_start_reg(&self) -> u32 {
        (self.active_video_start << 16) |
            (self.active_video_end)
    }

    pub fn write_h_start_reg(&mut self, value: u32) {
        self.active_video_start = (value >> 16) & 0x0000_03ff;
        self.active_video_end = value & 0x0000_03ff;
    }
}
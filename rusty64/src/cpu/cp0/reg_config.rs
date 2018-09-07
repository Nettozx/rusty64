#[derive(Debug)]
enum Ep { //page153 on datasheet
    D,
    DxxDxx,
    RFU
}

impl Default for Ep {
    fn default() -> Self {
        Ep::D
    }
}

#[derive(Debug)]
enum Be { //section 5.4.6
    LittleEndian,
    BigEndian
}

impl Default for Be {
    fn default() -> Self {
        Be::BigEndian
    }
}

#[derive(Default, Debug)]
pub struct RegConfig {
    ep: Ep,
    be: Be
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = Ep::D;
        self.be = Be::BigEndian;
    }
}
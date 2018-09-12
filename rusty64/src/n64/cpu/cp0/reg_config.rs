#[derive(Debug)]
enum DataTransferPattern { //page153 on datasheet
    Normal, //D
    DxxDxx, //DxxDxx
    //RFU //RFU - reserved for future use
}

impl Default for DataTransferPattern {
    fn default() -> Self {
        DataTransferPattern::Normal
    }
}

impl From<u32> for DataTransferPattern {
    fn from(value: u32) -> Self {
        match (value >> 24) & 0b1111 {
            0 => DataTransferPattern::Normal,
            6 => DataTransferPattern::DxxDxx,
            _ => panic!("Invalid data transfoer pattern (EP): {:#x}", value),
        }
    }
}

#[derive(Debug)]
enum Endianness { //section 5.4.6
    Little,
    Big,
}

impl Default for Endianness {
    fn default() -> Self {
        Endianness::Big
    }
}

impl From<u32> for Endianness {
    fn from(value: u32) -> Self {
        match (value >> 15) & 0b1 {
            0 => Endianness::Little,
            1 => Endianness::Big,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug)]
pub struct RegConfig {
    //EP
    data_transfer_pattern: DataTransferPattern,
    //BE
    endianness: Endianness,
    //CU
    cu: bool,
    //K0
    kseg0_cache_enable_bits: [bool;3],
}

impl RegConfig {
    fn kseg0_cache_enabled(&self) -> bool {
        !(!self.kseg0_cache_enable_bits[0] &&
            self.kseg0_cache_enable_bits[1] &&
            !self.kseg0_cache_enable_bits[2])
    }
}

impl From<u32> for RegConfig {
    fn from(value: u32) -> Self {
        //page 152 in datasheet
        RegConfig {
            //0 preset, EC unused
            // EP
            data_transfer_pattern: value.into(),
            //00000110 preset
            //BE
            endianness: value.into(),
            //11001000110 preset
            //CU
            cu: (value & (1 << 3)) != 0,
            //K0
            kseg0_cache_enable_bits: [
                (value & (1 << 0)) != 0,
                (value & (1 << 1)) != 0,
                (value & (1 << 2)) != 0,
            ],
        }
    }
}
const PIF_ROM_SIZE: usize = 2048;

//RAM on N64 is 4MB RDRAM with 9bit bus
const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect {
    pif_rom: Vec<u8>,
    ram: Vec<u16>
}

impl Interconnect {
    pub fn new(pif_rom: Vec<u8>) -> Interconnect {
        Interconnect {
            pif_rom: Vec::new(),
            ram: vec![0; RAM_SIZE],
        }
    }
}
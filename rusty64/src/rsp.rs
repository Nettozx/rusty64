use super::byteorder::{BigEndian, ByteOrder};
use super::mem_map::*;

pub struct Rsp {
    dmem: Box<[u8]>,
    imem: Box<[u8]>,

    halt: bool,
    broke: bool,
    interrupt_enable: bool,
}

impl Rsp {
    pub fn new() -> Rsp {
        //TODO check for correct initialization for hardware state
        Rsp {
            dmem: vec![0; SP_DMEM_LENGTH as usize].into_boxed_slice(),
            imem: vec![0; SP_IMEM_LENGTH as usize].into_boxed_slice(),

            halt: true,
            broke: false,
            interrupt_enable: false,
        }
    }

    pub fn read_dmem(&self, offset: u32) -> u32 {
        BigEndian::read_u32(&self.dmem[offset as usize..])
    }

    pub fn write_dmem(&mut self, offset: u32, value: u32) {
        BigEndian::write_u32(&mut self.dmem[offset as usize..], value)
    }

    pub fn read_imem(&self, offset: u32) -> u32 {
        BigEndian::read_u32(&self.imem[offset as usize..])
    }

    pub fn write_imem(&mut self, offset: u32, value: u32) {
        BigEndian::write_u32(&mut self.imem[offset as usize..], value)
    }

    pub fn read_status_reg(&self) -> u32 {
        (if self.halt {1} else {0} << 0) |
            (if self.interrupt_enable {1} else {0} << 1)
    }

    pub fn write_status_reg(&mut self, value: u32) {
        //TODO incomplete
        if (value & (1 << 0)) != 0 {
            self.halt = false;
        }
        if (value & (1 << 1)) != 0 {
            self.halt = true;
        }

        if (value & (1 << 2)) != 0 {
            self.broke = false;
        }
        if (value & (1 << 3)) != 0 {
            self.interrupt_enable = false;
        }

        if (value & 0xffff_fff0) != 0 {
            panic!("Write to unsupported rsp status bit: {:#?}", value);
        }
    }

    pub fn read_dma_busy_reg(&self) -> u32 {
        0//TODO
    }

    pub fn write_dma_busy_reg(&self, value: u32) {
        panic!("Attemted write to SP_DMA_BUSY_REG: {:#?}", value)
    }
}
use super::byteorder::{BigEndian, ByteOrder};
use super::mem_map::*;
use super::rsp::Rsp;

use std::fmt;

//RAM on N64 is 4MB RDRAM with 9bit bus
const RAM_SIZE: usize = 4 * 1024 * 1024;

enum Addr {
    PifRom(u32),
    SpStatusReg,
}

pub struct Interconnect {
    rsp: Rsp,
    pif_rom: Box<[u8]>,
    ram: Box<[u16]>
}

impl Interconnect {
    pub fn new(pif_rom: Box<[u8]>) -> Interconnect {
        Interconnect {
            rsp: Rsp::new(),
            pif_rom,
            ram: vec![0; RAM_SIZE].into_boxed_slice(),
        }
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        //look at n64 memory map txt for PIF_ROM start and end
        match self.mem_map(addr) {
            Addr::PifRom(offset) => BigEndian::read_u32( &self.pif_rom[offset as usize..]),
            Addr::SpStatusReg => self.rsp.read_status_reg()
        }
    }

    pub fn write_word(&mut self, addr: u32, value: u32) {
        match self.mem_map(addr) {
            Addr::PifRom(_) => panic!("Cannot write to PIF ROM"),
            Addr::SpStatusReg => self.rsp.write_status_reg(value)
        }
    }

    fn mem_map(&self, addr: u32) -> Addr {
        //look at n64 memory map txt for PIF_ROM start and end
        match addr {
            PIF_ROM_START...PIF_ROM_END => Addr::PifRom(addr - PIF_ROM_START),
            SP_STATUS_REG => Addr::SpStatusReg,
            _ => panic!("Unrecognized physical address: {:#x}", addr)
        }
    }
}

impl fmt::Debug for Interconnect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO: Impl Debug for Interconnect")
    }
}
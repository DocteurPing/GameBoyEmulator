use crate::graphics::{GPU, VRAM_BEGIN, VRAM_END};

pub(crate) struct MemoryBus {
    pub(crate) memory: [u8; 0xFFFF],
    pub(crate) graphics: GPU,
}

impl MemoryBus {
    pub(crate) fn read_byte(&self, address:u16) -> u8 {
        let address = address as usize;
        match address {
            VRAM_BEGIN ..= VRAM_END => {
                self.graphics.vram[address - VRAM_BEGIN]
            }
            _ => {
                // todo!();
                self.memory[address as usize]
            }
        }
    }

    pub(crate) fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                self.graphics.vram[addr - VRAM_BEGIN] = byte
            }
            _ => {
                self.memory[addr] = byte
            }
        }
    }
}
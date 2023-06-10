use crate::graphics::{empty_tile, GPU, VRAM_BEGIN, VRAM_END, VRAM_SIZE};
use crate::WINDOW_DIMENSIONS;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const ZERO_PAGE_BEGIN: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub(crate) struct MemoryBus {
    boot_rom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    pub(crate) graphics: GPU,
}

impl MemoryBus {
    pub(crate) fn read_byte(&self, address:u16) -> u8 {
        let address = address as usize;
        match address {
            BOOT_ROM_BEGIN ..= BOOT_ROM_END => {
                if let Some(boot_rom) = self.boot_rom {
                    boot_rom[address]
                } else {
                    self.rom_bank_0[address]
                }
            }
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                self.rom_bank_0[address]
            }
            ROM_BANK_N_BEGIN ..= ROM_BANK_N_END => {
                self.rom_bank_n[address - ROM_BANK_N_BEGIN]
            },
            EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
                self.external_ram[address - EXTERNAL_RAM_BEGIN]
            }
            WORKING_RAM_BEGIN ..= WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_BEGIN]
            },
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {
                self.working_ram[address - ECHO_RAM_BEGIN]
            },
            ZERO_PAGE_BEGIN ..= ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN]
            },
            VRAM_BEGIN ..= VRAM_END => {
                self.graphics.vram[address - VRAM_BEGIN]
            }
            _ => {
                panic!("TODO implement more memory spaces {}", address)
            }
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, byte: u8) {
        let address = address as usize;
        match address {
            ROM_BANK_0_BEGIN ..= ROM_BANK_0_END => {
                self.rom_bank_0[address] = byte
            }
            ROM_BANK_N_BEGIN ..= ROM_BANK_N_END => {
                self.rom_bank_n[address - ROM_BANK_N_BEGIN] = byte
            },
            EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
                self.external_ram[address - EXTERNAL_RAM_BEGIN] = byte
            }
            WORKING_RAM_BEGIN ..= WORKING_RAM_END => {
                self.working_ram[address - WORKING_RAM_BEGIN] = byte
            },
            ECHO_RAM_BEGIN ..= ECHO_RAM_END => {
                self.working_ram[address - ECHO_RAM_BEGIN] = byte
            },
            ZERO_PAGE_BEGIN ..= ZERO_PAGE_END => {
                self.zero_page[address - ZERO_PAGE_BEGIN] = byte
            },
            VRAM_BEGIN ..= VRAM_END => {
                self.graphics.vram[address - VRAM_BEGIN] = byte
            }
            _ => {
                panic!("TODO implement more memory spaces {}", address)
            }
        }
    }

    pub fn new(boot_rom_buffer: Option<Vec<u8>>, game_rom: Vec<u8>) -> MemoryBus {
        let boot_rom = boot_rom_buffer.map(|boot_rom_buffer| {
            if boot_rom_buffer.len() != BOOT_ROM_SIZE {
                panic!(
                    "Supplied boot ROM is the wrong size. Is {} bytes but should be {} bytes",
                    boot_rom_buffer.len(),
                    BOOT_ROM_SIZE
                );
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&boot_rom_buffer);
            boot_rom
        });

        let mut rom_bank_0 = [0; ROM_BANK_0_SIZE];
        let mut rom_bank_n = [0; ROM_BANK_N_SIZE];
        for i in 0..ROM_BANK_0_SIZE {
            rom_bank_0[i] = game_rom[i];
        }
        for i in 0..ROM_BANK_N_SIZE {
            rom_bank_n[i] = game_rom[ROM_BANK_0_SIZE + i];
        }
        MemoryBus {
            // Note: instead of modeling memory as one array of length 0xFFFF, we'll
            // break memory up into it's logical parts.
            boot_rom,
            rom_bank_0,
            rom_bank_n,
            external_ram: [0; EXTERNAL_RAM_SIZE],
            working_ram: [0; WORKING_RAM_SIZE],
            zero_page: [0; ZERO_PAGE_SIZE],
            graphics: GPU {
                vram: [0; VRAM_SIZE],
                tile_set: [empty_tile(); 384],
                canvas_buffer: [u32::MAX; WINDOW_DIMENSIONS[0] * WINDOW_DIMENSIONS[1] * 4],
            },
        }
    }
}
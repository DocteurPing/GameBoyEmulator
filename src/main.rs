extern crate clap;

mod graphics;
mod memory;
mod utils;
mod cpu;

use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};
use crate::cpu::{CPU, FlagRegister, Registers};
use crate::graphics::{empty_tile, GPU, VRAM_SIZE};
use crate::memory::MemoryBus;
use crate::utils::buffer_from_file;

fn main() {
    let args = App::new("Emulator")
        .arg(Arg::with_name("boot").short("b"))
        .arg(Arg::with_name("rom").short("r"))
        .get_matches();
    let boot = args.value_of("boot").map(|path| buffer_from_file(path));
    let rom = args.value_of("rom").map(|path| buffer_from_file(path));
    let cpu = CPU {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        },
        pc: 0x0,
        sp: 0x00,
        bus: MemoryBus {
            memory: [0; 0xFFFF],
            graphics: GPU {
                vram: [0; VRAM_SIZE],
                tile_set: [empty_tile(); 384],
            },
        },
        is_halted: false,
    };
    println!("Hello, world!");
}

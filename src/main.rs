extern crate clap;

mod graphics;
mod memory;
mod utils;
mod cpu;

use std::thread::sleep;
use std::time::{Duration, Instant};
use clap::{App, Arg};
use minifb::{Key, Window, WindowOptions};
use crate::cpu::{CPU, FlagRegister, Registers};
use crate::graphics::{empty_tile, GPU, VRAM_SIZE};
use crate::memory::MemoryBus;
use crate::utils::buffer_from_file;

const ENLARGEMENT_FACTOR: usize = 1;
const WINDOW_DIMENSIONS: [usize; 2] = [(160 * ENLARGEMENT_FACTOR), (144 * ENLARGEMENT_FACTOR)];
const NUMBER_OF_PIXELS: usize = 23040;
const ONE_FRAME_IN_CYCLES: usize = 70224;

fn main() {
    // let args = App::new("Emulator")
    //     .arg(Arg::with_name("boot").short("b"))
    //     .arg(Arg::with_name("rom").short("r"))
    //     .get_matches();
    // let boot = args.value_of("boot").map(|path| buffer_from_file(path));
    // let rom = args.value_of("rom").map(|path| buffer_from_file(path));
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
                canvas_buffer: [u32::MAX; WINDOW_DIMENSIONS[0] * WINDOW_DIMENSIONS[1] * 4],
            },
        },
        is_halted: false,
    };
    let window = Window::new("Emulator", WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1], WindowOptions::default()).unwrap();
    run(cpu, window);
}

fn run(cpu: CPU, mut window: Window) {
    let mut buffer: [u32; NUMBER_OF_PIXELS] = [0; NUMBER_OF_PIXELS];
    let mut cycles: usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cycles += 1; // cpu.run()
        if cycles >= ONE_FRAME_IN_CYCLES {
            for (i, pixel) in cpu.bus.graphics.canvas_buffer.chunks(4).enumerate() {
                buffer[i] = (pixel[3] as u32) << 24
                    | (pixel[2] as u32) << 16
                    | (pixel[1] as u32) << 8
                    | (pixel[0] as u32)
            }
            window.update_with_buffer(&buffer, WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1]).unwrap();
            cycles = 0;
        } else {
            sleep(Duration::from_nanos(2));
        }
    }
}

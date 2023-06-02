use crate::cpu::register::Registers;
use crate::cpu::instructions::Instruction;
use crate::cpu::instructions::ArithmeticTarget;

mod register;
mod instructions;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_bytes(&self, address:u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    fn step(&mut self) {
        let instruction_byte = self.bus.read_bytes(self.pc);
        if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.pc = self.execute(instruction);
        } else {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        }
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => self.add(self.registers.a),
                    ArithmeticTarget::B => self.add(self.registers.b),
                    ArithmeticTarget::C => self.add(self.registers.c),
                    ArithmeticTarget::D => self.add(self.registers.d),
                    ArithmeticTarget::E => self.add(self.registers.e),
                    ArithmeticTarget::H => self.add(self.registers.h),
                    ArithmeticTarget::L => self.add(self.registers.l),
                }
            }
            _ => {}
        }
        0
    }
    fn add(&mut self, nbr: u8) {
        let (result, overflow) = self.registers.a.overflowing_add(nbr);
        self.registers.a = result;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (result & 0xF) > 0xF;
    }
}
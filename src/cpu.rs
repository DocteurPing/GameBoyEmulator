use crate::cpu::register::Registers;
use crate::cpu::instructions::{Instruction, JumpTest};
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
    fn read_byte(&self, address:u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        // Check if it's a prefix byte
        let is_prefix =  instruction_byte == 0xCB;
        if is_prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefix) {
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
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true
                };
                self.jump(jump_condition)
            }
            _ => self.pc
        }
    }

    fn add(&mut self, nbr: u8) -> u16 {
        let (result, overflow) = self.registers.a.overflowing_add(nbr);
        self.registers.a = result;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (result & 0xF) > 0xF;
        self.pc.wrapping_add(1)
    }

    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian so read pc + 2 as most significant bit
            // and pc + 1 as least significant bit
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            // If we don't jump we need to still move the program
            // counter forward by 3 since the jump instruction is
            // 3 bytes wide (1 byte for tag and 2 bytes for jump address)
            self.pc.wrapping_add(3)
        }
    }
}
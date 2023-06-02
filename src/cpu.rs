use crate::cpu::register::Registers;
use crate::cpu::instructions::Instruction;
use crate::cpu::instructions::ArithmeticTarget;

mod register;
mod instructions;

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
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
        }
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
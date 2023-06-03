use crate::cpu::register::Registers;
use crate::cpu::instructions::{Instruction, JumpTest, LoadByteSource, LoadByteTarget, LoadType, MultipleBytesRegister};
use crate::cpu::instructions::AddTarget;
use crate::memory::MemoryBus;

mod register;
mod instructions;

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
    sp: u16,
    is_halted: bool,
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
        if self.is_halted {
            return self.pc;
        }
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    AddTarget::A => self.add(self.registers.a),
                    AddTarget::B => self.add(self.registers.b),
                    AddTarget::C => self.add(self.registers.c),
                    AddTarget::D => self.add(self.registers.d),
                    AddTarget::E => self.add(self.registers.e),
                    AddTarget::H => self.add(self.registers.h),
                    AddTarget::L => self.add(self.registers.l),
                    AddTarget::HLI => self.add(self.bus.read_byte(self.registers.get_hl())),
                    AddTarget::D8 => self.add(self.read_next_byte())
                };
                self.pc.wrapping_add(1)
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
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                        };
                        match source {
                            LoadByteSource::D8  => self.pc.wrapping_add(2),
                            _                   => self.pc.wrapping_add(1),
                        }
                    }
                    _ => { panic!("TODO: implement other load types") }
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    MultipleBytesRegister::BC => self.registers.get_bc(),
                    MultipleBytesRegister::AF => self.registers.get_af(),
                    MultipleBytesRegister::DE => self.registers.get_de(),
                    MultipleBytesRegister::HL => self.registers.get_hl(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let value = self.pop();
                match target {
                    MultipleBytesRegister::AF => self.registers.set_af(value),
                    MultipleBytesRegister::BC => self.registers.set_bc(value),
                    MultipleBytesRegister::DE => self.registers.set_de(value),
                    MultipleBytesRegister::HL => self.registers.set_hl(value),
                }

                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    _ => { panic!("TODO: support more conditions") }
                };
                self.return_(jump_condition)
            }
            Instruction::NOP() => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT() => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            _ => self.pc
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

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, value as u8);
    }

    fn pop(&mut self) -> u16 {
        let last_byte: u16 = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let first_byte: u16 = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        first_byte << 8 | last_byte
    }

    fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_word(&self) -> u16 {
        let last_byte = self.bus.read_byte(self.pc + 1) as u16;
        let first_byte = self.bus.read_byte(self.pc + 2) as u16;
        first_byte << 8 | last_byte
    }
}
mod register;
mod instructions;

pub(crate) use crate::cpu::register::{Registers, FlagRegister};
use crate::cpu::instructions::{Instruction, JumpTest, LoadByteSource, LoadByteTarget, LoadType, LoadWordTarget, MultipleBytesRegister, PrefixTarget};
use crate::cpu::instructions::ArithmeticTarget;
use crate::memory::MemoryBus;

pub(crate) struct CPU {
    pub(crate) registers: Registers,
    pub(crate) pc: u16,
    pub(crate) bus: MemoryBus,
    pub(crate) sp: u16,
    pub(crate) is_halted: bool,
}

impl CPU {
    pub(crate) fn step(&mut self) -> u16 {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        // Check if it's a prefix byte
        let is_prefix = instruction_byte == 0xCB;
        if is_prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }
        if let Some(instruction) = Instruction::from_byte(instruction_byte, is_prefix) {
            self.pc = self.execute(instruction);
        } else {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        }
        return self.pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        if self.is_halted {
            return self.pc;
        }
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
                    ArithmeticTarget::HLI => self.add(self.bus.read_byte(self.registers.get_hl())),
                    ArithmeticTarget::D8 => self.add(self.read_next_byte())
                };
                self.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => self.adc(self.registers.a),
                    ArithmeticTarget::B => self.adc(self.registers.b),
                    ArithmeticTarget::C => self.adc(self.registers.c),
                    ArithmeticTarget::D => self.adc(self.registers.d),
                    ArithmeticTarget::E => self.adc(self.registers.e),
                    ArithmeticTarget::H => self.adc(self.registers.h),
                    ArithmeticTarget::L => self.adc(self.registers.l),
                    ArithmeticTarget::HLI => self.adc(self.bus.read_byte(self.registers.get_hl())),
                    ArithmeticTarget::D8 => self.adc(self.read_next_byte())
                };
                self.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => self.sub(self.registers.a),
                    ArithmeticTarget::B => self.sub(self.registers.b),
                    ArithmeticTarget::C => self.sub(self.registers.c),
                    ArithmeticTarget::D => self.sub(self.registers.d),
                    ArithmeticTarget::E => self.sub(self.registers.e),
                    ArithmeticTarget::H => self.sub(self.registers.h),
                    ArithmeticTarget::L => self.sub(self.registers.l),
                    ArithmeticTarget::HLI => self.sub(self.bus.read_byte(self.registers.get_hl())),
                    ArithmeticTarget::D8 => self.sub(self.read_next_byte())
                };
                self.pc.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => self.sbc(self.registers.a),
                    ArithmeticTarget::B => self.sbc(self.registers.b),
                    ArithmeticTarget::C => self.sbc(self.registers.c),
                    ArithmeticTarget::D => self.sbc(self.registers.d),
                    ArithmeticTarget::E => self.sbc(self.registers.e),
                    ArithmeticTarget::H => self.sbc(self.registers.h),
                    ArithmeticTarget::L => self.sbc(self.registers.l),
                    ArithmeticTarget::HLI => self.sbc(self.bus.read_byte(self.registers.get_hl())),
                    ArithmeticTarget::D8 => self.sbc(self.read_next_byte())
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
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1),
                        }
                    }
                    LoadType::Word(target) => {
                        let word = self.read_next_word();
                        match target {
                            LoadWordTarget::BC => { self.registers.set_bc(word) }
                            LoadWordTarget::DE => { self.registers.set_de(word) }
                            LoadWordTarget::HL => { self.registers.set_hl(word) }
                            LoadWordTarget::SP => { self.sp = word }
                        };
                        self.pc.wrapping_add(3)
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
            Instruction::SWAP(target) => {
                match target {
                    PrefixTarget::A => { self.registers.a = self.swap_nibbles(self.registers.a) }
                    PrefixTarget::B => { self.registers.b = self.swap_nibbles(self.registers.b) }
                    PrefixTarget::C => { self.registers.c = self.swap_nibbles(self.registers.c) }
                    PrefixTarget::D => { self.registers.d = self.swap_nibbles(self.registers.d) }
                    PrefixTarget::E => { self.registers.e = self.swap_nibbles(self.registers.e) }
                    PrefixTarget::H => { self.registers.h = self.swap_nibbles(self.registers.h) }
                    PrefixTarget::L => { self.registers.l = self.swap_nibbles(self.registers.l) }
                    PrefixTarget::HLI => {
                        let value = self.swap_nibbles(self.bus.read_byte(self.registers.get_hl()));
                        self.bus.write_byte(self.registers.get_hl(), value)
                    }
                }
                self.pc.wrapping_add(2)
            }
            _ => panic!("TODO: support more instructions")
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

    fn adc(&mut self, nbr: u8) {
        let nbr = if self.registers.f.carry {
            nbr
        } else {
            nbr + 1
        };
        self.add(nbr);
    }

    fn sub(&mut self, nbr: u8) {
        let (result, overflow) = self.registers.a.overflowing_sub(nbr);
        self.registers.a = result;
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (result & 0xF);
    }

    fn sbc(&mut self, nbr: u8) {
        let nbr = if self.registers.f.carry {
            nbr
        } else {
            nbr + 1
        };
        self.sub(nbr);
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

    fn swap_nibbles(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
    }
}
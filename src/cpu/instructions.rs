pub(crate) enum Instruction {
    ADD(AddTarget),
    INC(IncTarget),
    RLC(PrefixTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(MultipleBytesRegister),
    POP(MultipleBytesRegister),
    CALL(JumpTest),
    RET(JumpTest),
    NOP(),
    HALT(),
}

pub(crate) enum AddTarget {
    A, B, C, D, E, H, L, HLI, D8
}

pub(crate) enum IncTarget {
    A, B, C, D, E, H, L, HLI, BC, DE, HL, SP
}

pub(crate) enum PrefixTarget {
    A, B, C, D, E, H, L,
}

pub(crate) enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always
}

pub(crate) enum LoadByteTarget {
    A, B, C, D, E, H, L, HLI
}

pub(crate) enum LoadByteSource {
    A, B, C, D, E, H, L, D8, HLI
}

pub(crate) enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

pub(crate) enum MultipleBytesRegister {
    AF, BC, DE, HL
}

impl Instruction {
    pub(crate) fn from_byte(byte: u8, is_prefix: bool) -> Option<Instruction> {
        if is_prefix {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }

    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x3c => Some(Instruction::INC(IncTarget::A)),
            0x04 => Some(Instruction::INC(IncTarget::B)),
            0x14 => Some(Instruction::INC(IncTarget::D)),
            0x24 => Some(Instruction::INC(IncTarget::H)),
            0x0c => Some(Instruction::INC(IncTarget::C)),
            0x1c => Some(Instruction::INC(IncTarget::E)),
            0x2c => Some(Instruction::INC(IncTarget::L)),
            0x34 => Some(Instruction::INC(IncTarget::HLI)),
            0x03 => Some(Instruction::INC(IncTarget::BC)),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            0x23 => Some(Instruction::INC(IncTarget::HL)),
            0x33 => Some(Instruction::INC(IncTarget::SP)),

            0x87 => Some(Instruction::ADD(AddTarget::A)),
            0x80 => Some(Instruction::ADD(AddTarget::B)),
            0x81 => Some(Instruction::ADD(AddTarget::C)),
            0x82 => Some(Instruction::ADD(AddTarget::D)),
            0x83 => Some(Instruction::ADD(AddTarget::E)),
            0x84 => Some(Instruction::ADD(AddTarget::H)),
            0x85 => Some(Instruction::ADD(AddTarget::L)),
            0x86 => Some(Instruction::ADD(AddTarget::HLI)),
            0xc6 => Some(Instruction::ADD(AddTarget::D8)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
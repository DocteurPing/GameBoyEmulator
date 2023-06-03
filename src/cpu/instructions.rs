pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
    INC(ExtendedRegisterTarget),
    RLC(PrefixTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(MultipleBytesRegister),
    POP(MultipleBytesRegister)
}

pub(crate) enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub(crate) enum ExtendedRegisterTarget {
    A, B, C, D, E, H, L, AF, BC, DE, HL
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
            0x02 => Some(Instruction::INC(ExtendedRegisterTarget::BC)),
            0x13 => Some(Instruction::INC(ExtendedRegisterTarget::DE)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
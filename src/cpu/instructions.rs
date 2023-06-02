pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncTarget),
    RLC(PrefixTarget),
    JP(JumpTest)
}

pub(crate) enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub(crate) enum IncTarget {
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
            0x02 => Some(Instruction::INC(IncTarget::BC)),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
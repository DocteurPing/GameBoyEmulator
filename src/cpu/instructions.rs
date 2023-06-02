pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncTarget),
}

pub(crate) enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

pub enum IncTarget {
    A, B, C, D, E, H, L, AF, BC, DE, HL
}

impl Instruction {
    pub(crate) fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncTarget::BC)),
            0x13 => Some(Instruction::INC(IncTarget::DE)),
            _ => None
        }
    }
}
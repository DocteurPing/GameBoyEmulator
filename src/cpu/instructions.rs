pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
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
    SWAP(PrefixTarget),
    XOR(ArithmeticTarget)
}

pub(crate) enum ArithmeticTarget {
    A, B, C, D, E, H, L, HLI, D8
}

pub(crate) enum IncTarget {
    A, B, C, D, E, H, L, HLI, BC, DE, HL, SP
}

pub(crate) enum PrefixTarget {
    A, B, C, D, E, H, L, HLI
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
    Word(LoadWordTarget),
    IndirectFromA(Indirect)
}

pub(crate) enum LoadWordTarget {
    BC, DE, HL, SP
}

pub(crate) enum MultipleBytesRegister {
    AF, BC, DE, HL
}

pub(crate) enum Indirect {
    BCIndirect,
    DEIndirect,
    HLIndirectMinus,
    HLIndirectPlus,
    WordIndirect,
    LastByteIndirect,
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
            0x30 => Some(Instruction::SWAP(PrefixTarget::B)),
            0x31 => Some(Instruction::SWAP(PrefixTarget::C)),
            0x32 => Some(Instruction::SWAP(PrefixTarget::D)),
            0x33 => Some(Instruction::SWAP(PrefixTarget::E)),
            0x34 => Some(Instruction::SWAP(PrefixTarget::H)),
            0x35 => Some(Instruction::SWAP(PrefixTarget::L)),
            0x36 => Some(Instruction::SWAP(PrefixTarget::HLI)),
            0x37 => Some(Instruction::SWAP(PrefixTarget::A)),

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

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x86 => Some(Instruction::ADD(ArithmeticTarget::HLI)),
            0xc6 => Some(Instruction::ADD(ArithmeticTarget::D8)),

            0x8f => Some(Instruction::ADC(ArithmeticTarget::A)),
            0x88 => Some(Instruction::ADC(ArithmeticTarget::B)),
            0x89 => Some(Instruction::ADC(ArithmeticTarget::C)),
            0x8a => Some(Instruction::ADC(ArithmeticTarget::D)),
            0x8b => Some(Instruction::ADC(ArithmeticTarget::E)),
            0x8c => Some(Instruction::ADC(ArithmeticTarget::H)),
            0x8d => Some(Instruction::ADC(ArithmeticTarget::L)),
            0x8e => Some(Instruction::ADC(ArithmeticTarget::HLI)),
            0xce => Some(Instruction::ADC(ArithmeticTarget::D8)),

            0x97 => Some(Instruction::SUB(ArithmeticTarget::A)),
            0x90 => Some(Instruction::SUB(ArithmeticTarget::B)),
            0x91 => Some(Instruction::SUB(ArithmeticTarget::C)),
            0x92 => Some(Instruction::SUB(ArithmeticTarget::D)),
            0x93 => Some(Instruction::SUB(ArithmeticTarget::E)),
            0x94 => Some(Instruction::SUB(ArithmeticTarget::H)),
            0x95 => Some(Instruction::SUB(ArithmeticTarget::L)),
            0x96 => Some(Instruction::SUB(ArithmeticTarget::HLI)),
            0xd6 => Some(Instruction::SUB(ArithmeticTarget::D8)),

            0x9f => Some(Instruction::SBC(ArithmeticTarget::A)),
            0x98 => Some(Instruction::SBC(ArithmeticTarget::B)),
            0x99 => Some(Instruction::SBC(ArithmeticTarget::C)),
            0x9a => Some(Instruction::SBC(ArithmeticTarget::D)),
            0x9b => Some(Instruction::SBC(ArithmeticTarget::E)),
            0x9c => Some(Instruction::SBC(ArithmeticTarget::H)),
            0x9d => Some(Instruction::SBC(ArithmeticTarget::L)),
            0x9e => Some(Instruction::SBC(ArithmeticTarget::HLI)),
            0xde => Some(Instruction::SBC(ArithmeticTarget::D8)),

            0x01 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::BC))),
            0x11 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::DE))),
            0x21 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::HL))),
            0x31 => Some(Instruction::LD(LoadType::Word(LoadWordTarget::SP))),

            0xaf => Some(Instruction::XOR(ArithmeticTarget::A)),
            0xa8 => Some(Instruction::XOR(ArithmeticTarget::B)),
            0xa9 => Some(Instruction::XOR(ArithmeticTarget::C)),
            0xaa => Some(Instruction::XOR(ArithmeticTarget::D)),
            0xab => Some(Instruction::XOR(ArithmeticTarget::E)),
            0xac => Some(Instruction::XOR(ArithmeticTarget::H)),
            0xad => Some(Instruction::XOR(ArithmeticTarget::L)),
            0xae => Some(Instruction::XOR(ArithmeticTarget::HLI)),
            0xee => Some(Instruction::XOR(ArithmeticTarget::D8)),

            0xe2 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::LastByteIndirect))),
            0x02 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::BCIndirect))),
            0x12 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::DEIndirect))),
            0x22 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectPlus))),
            0x32 => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::HLIndirectMinus))),
            0xea => Some(Instruction::LD(LoadType::IndirectFromA(Indirect::WordIndirect))),

            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}
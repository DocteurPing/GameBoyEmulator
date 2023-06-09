const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub(crate) struct Registers {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: FlagRegister,
    pub(crate) h: u8,
    pub(crate) l: u8,
}

#[derive(Copy, Clone)]
pub(crate) struct FlagRegister {
    pub(crate) zero: bool,
    pub(crate) subtract: bool,
    pub(crate) half_carry: bool,
    pub(crate) carry: bool,
}

impl Registers {
    pub(crate) fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    pub(crate) fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub(crate) fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub(crate) fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub(crate) fn set_af(&mut self, af: u16) {
        self.a = (af >> 8) as u8;
        self.f = (af as u8).into();
    }

    pub(crate) fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = bc as u8;
    }

    pub(crate) fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = de as u8;
    }

    pub(crate) fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;
    }
}

impl std::convert::From<FlagRegister> for u8  {
    fn from(flag: FlagRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
            (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
            (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
            (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte >> ZERO_FLAG_BYTE_POSITION) != 0;
        let subtract = (byte >> SUBTRACT_FLAG_BYTE_POSITION) != 0;
        let half_carry = (byte >> HALF_CARRY_FLAG_BYTE_POSITION) != 0;
        let carry = (byte >> CARRY_FLAG_BYTE_POSITION) != 0;

        FlagRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_af(&mut self, af: u16) {
        self.a = (af >> 8) as u8;
        self.f = af as u8;
    }

    fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = bc as u8;
    }

    fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = de as u8;
    }
    fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;
    }
}
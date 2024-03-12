use super::flagsregister::FlagsRegister;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister::from(0u8),
            h: 0,
            l: 0,
        }
    }

    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn c(&self) -> u8 {
        self.c
    }

    pub fn d(&self) -> u8 {
        self.d
    }

    pub fn e(&self) -> u8 {
        self.e
    }

    pub fn f(&self) -> &FlagsRegister {
        &self.f
    }

    pub fn f_as_mut(&mut self) -> &mut FlagsRegister {
        &mut self.f
    }

    pub fn h(&self) -> u8 {
        self.h
    }

    pub fn l(&self) -> u8 {
        self.l
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }

    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | (u16::from(self.f))
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagsRegister::from(value as u8);
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hl() {
        let mut registers = Registers::new();
        assert_eq!(registers.hl(), 0);
        registers.set_hl(42);
        assert_eq!(registers.hl(), 42);
        registers.set_hl(0xFFEE);
        assert_eq!(registers.hl(), 0xFFEE);
        assert_eq!(registers.l(), 0xEE);
        assert_eq!(registers.h(), 0xFF);
    }
    #[test]
    fn de() {
        let mut registers = Registers::new();
        assert_eq!(registers.de(), 0);
        registers.set_de(42);
        assert_eq!(registers.de(), 42);
        registers.set_de(0xFFEE);
        assert_eq!(registers.de(), 0xFFEE);
        assert_eq!(registers.e(), 0xEE);
        assert_eq!(registers.d(), 0xFF);
    }
}

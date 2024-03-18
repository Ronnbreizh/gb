const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
/// CHECKME
const EMI_FLAG_BYTE_POSITION: u8 = 3;

#[derive(Copy, Clone, Debug)]
/// Contain all the CPU flags.
/// TODO: Remove the U8 conversion
pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
    /// Interrupt Master Enabled flag
    emi: bool,
}

impl FlagsRegister {
    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn subtract(&self) -> bool {
        self.subtract
    }

    pub fn half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn emi(&self) -> bool {
        self.emi
    }

    pub fn set_zero(&mut self, zero: bool) {
        self.zero = zero
    }

    pub fn set_subtract(&mut self, subtract: bool) {
        self.subtract = subtract
    }

    pub fn set_half_carry(&mut self, half_carry: bool) {
        self.half_carry = half_carry
    }

    pub fn set_carry(&mut self, carry: bool) {
        self.carry = carry
    }

    pub fn set_emi(&mut self, interrupt_enabled: bool) {
        self.emi = interrupt_enabled;
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
            | (if flag.emi { 1 } else { 0 } << EMI_FLAG_BYTE_POSITION)
    }
}

impl std::convert::From<FlagsRegister> for u16 {
    fn from(flags: FlagsRegister) -> u16 {
        <FlagsRegister as Into<u8>>::into(flags) as u16
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let emi = ((byte >> EMI_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
            emi,
        }
    }
}

impl std::convert::From<u16> for FlagsRegister {
    fn from(byte: u16) -> Self {
        FlagsRegister::from(byte as u8)
    }
}

use std::ops::RangeInclusive;

pub trait Memory {
    /// begin of memory
    fn start() -> u16;
    /// end of memory
    fn end() -> u16;

    /// range
    fn range() -> RangeInclusive<u16> {
        Self::start()..=Self::end()
    }

    /// read buffer
    fn buffer(&self) -> &[u8];

    /// get buffer for edition purposes
    fn buffer_as_mut(&mut self) -> &mut [u8];

    /// write byte
    fn write_byte(&mut self, address: u16, content : u8) {
        let address = Self::local_address(address);

        self.buffer_as_mut()[address as usize] = content;
    }

    /// write word
    fn write_word(&mut self, address: u16, content: u16) {
        let address = Self::local_address(address);
        // lower part
        self.buffer_as_mut()[address as usize] = (content ^ 0xFF )as u8;
        // higher part
        self.buffer_as_mut()[(address + 1) as usize] = (content >> 8) as u8;
    }


    /// transmute global address into local address
    fn local_address(address : u16) -> u16 {
        address - Self::start()
    }

    /// read byte
    /// address is in the whole memory space (0x0000 -> 0xFFFF)
    fn read_byte(&self, address: u16) -> u8 {
        let address = Self::local_address(address);

        self.buffer()[address as usize]
    }

    /// read word = 2 bytes
    fn read_word(&self, address: u16) -> u16 {
        let address = Self::local_address(address);

        // beware of the endianess
        (self.buffer()[address as usize] as u16) | ((self.buffer()[(address +1 )as usize] as u16) << 4)
    }
}

#[cfg(test)]
mod test {
    use super::Memory;

    struct TestMemory {
        buffer: [u8;5]
    }

    impl Memory for TestMemory {
        fn start() -> u16 {
            4
        }

        fn end() -> u16 {
            9
        }

        fn buffer(&self) -> &[u8] {
            &self.buffer
        }

        fn buffer_as_mut(&mut self) -> &mut [u8] {
            &mut self.buffer
        }
    }

    #[test]
    fn test_memory() {
        let buffer = [1,2,3,4,5];
        let mut memory = TestMemory{buffer};

        assert_eq!(memory.read_byte(4), 1);
        assert_eq!(memory.read_byte(8), 5);

        memory.write_byte(5, 42);
        assert_eq!(memory.read_byte(5), 42)
    }

    #[test]
    #[should_panic]
    fn test_invalid_write() {
        let buffer = [1,2,3,4,5];
        let mut memory = TestMemory{buffer};

        memory.write_byte(2, 23);
    }

    #[test]
    #[should_panic]
    fn test_invalid_read() {
        let buffer = [1,2,3,4,5];
        let memory = TestMemory{buffer};

        memory.read_byte(32);
    }
}
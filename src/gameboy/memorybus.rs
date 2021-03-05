use std::fs::File;
use std::io::Read;

type Memory = [u8; 0xFFFF];

#[derive(Debug)]
pub struct MemoryBus {
    memory: Memory 
}

impl MemoryBus {
    pub fn new() -> Self {
        let mut memory = [0u8; 0xFFFF];
        Self::load_boot(&mut memory);

        Self {
            memory,
        }   
    }

    pub fn load(rom_path: &str) -> Self {
        let mut memory = [0u8; 0xFFFF];
        Self::load_boot(&mut memory);

        Self {
            memory,
        }   
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_2_bytes(&self, address: u16) -> u16 {
        (self.memory[address as usize] as u16) |
        ((self.memory[(address+1) as usize] as u16) << 8)
    }

    pub fn vram(&self) -> Vec<u8> {
        // allocation that may be removed
       vec![0x4; 144*160*3] 
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write_2_bytes(&mut self, address: u16, value: u16) {
        //TODO
        unimplemented!("Writing two bytes")
    }

    pub fn load_boot(memory : &mut Memory){
        let mut boot_sequence = File::open("etc/DMG_ROM.bin").unwrap();

        boot_sequence.read(memory);
    }
}

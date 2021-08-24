use std::fs::File;
use std::io::Read;

use crate::gameboy::memory::{BANK_0_END, BANK_0_START, BANK_1_END, BANK_1_START, ECHO_RAM_END, ECHO_RAM_START, HIGH_RAM_END, HIGH_RAM_START, INTERRUPTS_REGISTER, IO_REGISTER_END, IO_REGISTER_START, SPRITE_TABLE_END, SPRITE_TABLE_START, VRAM_END, VRAM_START};
use crate::gameboy::memory::memory_behavior::Memory;

use super::{BOOT_SEQUENCE_PATH, BOOT_SEQUENCE_SIZE, ROM_END, ROM_START, EXT_RAM_START, EXT_RAM_END};
use super::memory_zone::{Bank0, Bank1, EchoRam, ExternalRam, HighRam, InterruptsRegister, IoRegister, ReadOnlyMemory, SpriteAttributeTable, VideoRam};

#[derive(Debug, Default)]
/// The memory bus is the shared memory of the GB
/// Components can communicate via this bus
/// TODO use RwLock on each component when getting multithreaded
pub struct MemoryBus {
    /// ROM : boosequence + cartridge game
    read_only_memory: ReadOnlyMemory,
    /// Buffer for display
    video_ram: VideoRam,
    /// RAM brought by the cartridge
    external_ram: ExternalRam,
    /// Work RAM. Does not exist in GB
    bank_0 : Bank0,
    /// Work RAM. Presents with GB and CGB
    bank_1 : Bank1,
    /// Echo RAM. Usually not used
    echo_ram : EchoRam,
    /// Sprite attribute table
    sprite_attribute_table : SpriteAttributeTable,
    /// IO register to store inputs, sound etc
    io_register : IoRegister,
    /// Yet an other RAM
    high_ram : HighRam,
    /// Interrupt register
    interrupt_register : InterruptsRegister,
}

impl MemoryBus {
    /// Load bus and cartrige
    pub fn load(rom_path: &str) -> Self {
        let mut memory_bus = MemoryBus::default();
        memory_bus.load_boot();
        memory_bus.load_cartridge(rom_path);
        memory_bus
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.read_byte(address),
            VRAM_START..=VRAM_END => self.video_ram.read_byte(address),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.read_byte(address),
            BANK_0_START..=BANK_0_END => self.bank_0.read_byte(address),
            BANK_1_START..=BANK_1_END => self.bank_1.read_byte(address),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.read_byte(address),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self.sprite_attribute_table.read_byte(address),
            IO_REGISTER_START..=IO_REGISTER_END => self.io_register.read_byte(address),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.read_byte(address),
            INTERRUPTS_REGISTER => self.interrupt_register.read_byte(address),
            _ => panic!("Address {} not handled", address),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.read_word(address),
            VRAM_START..=VRAM_END => self.video_ram.read_word(address),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.read_word(address),
            BANK_0_START..=BANK_0_END => self.bank_0.read_word(address),
            BANK_1_START..=BANK_1_END => self.bank_1.read_word(address),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.read_word(address),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self.sprite_attribute_table.read_word(address),
            IO_REGISTER_START..=IO_REGISTER_END => self.io_register.read_word(address),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.read_word(address),
            INTERRUPTS_REGISTER => self.interrupt_register.read_word(address),
            _ => panic!("Address {} not handled", address),
        }
    }

    /// return video ram buffer
    pub fn vram(&self) -> &[u8] {
        self.video_ram.buffer()
    }

    /// write byte to memory
    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.write_byte(address, value),
            VRAM_START..=VRAM_END => self.video_ram.write_byte(address, value),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.write_byte(address, value),
            BANK_0_START..=BANK_0_END => self.bank_0.write_byte(address, value),
            BANK_1_START..=BANK_1_END => self.bank_1.write_byte(address, value),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.write_byte(address, value),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self.sprite_attribute_table.write_byte(address,value ),
            IO_REGISTER_START..=IO_REGISTER_END => self.io_register.write_byte(address, value),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.write_byte(address, value),
            INTERRUPTS_REGISTER => self.interrupt_register.write_byte(address, value),
            _ => panic!("Address {} not handled", address),
        }
    }

    /// write word to memory
    pub fn write_word(&mut self, address: u16, value: u16) {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.write_word(address, value),
            VRAM_START..=VRAM_END => self.video_ram.write_word(address, value),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.write_word(address, value),
            BANK_0_START..=BANK_0_END => self.bank_0.write_word(address, value),
            BANK_1_START..=BANK_1_END => self.bank_1.write_word(address, value),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.write_word(address, value),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self.sprite_attribute_table.write_word(address,value ),
            IO_REGISTER_START..=IO_REGISTER_END => self.io_register.write_word(address, value),
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.write_word(address, value),
            INTERRUPTS_REGISTER => self.interrupt_register.write_word(address, value),
            _ => panic!("Address {} not handled", address),
        }
    }

    /// Load boot sequence to the beginning of the ROM
    pub fn load_boot(&mut self){
        let memory = &mut self.read_only_memory;
        let mut boot_sequence = File::open(BOOT_SEQUENCE_PATH).unwrap();

        boot_sequence.read(memory.buffer_as_mut());
    }

    /// Load cartridge after the boot sequence.
    pub fn load_cartridge(&mut self, cartrige_path: &str) {
        let memory = &mut self.read_only_memory;
        let mut cartridge = File::open(cartrige_path).unwrap();

        // write content in memoy after boot sequence size
        cartridge.read(&mut memory.buffer_as_mut()[BOOT_SEQUENCE_SIZE..]).unwrap();
    }
}

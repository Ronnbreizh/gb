use std::fs::File;
use std::io::Read;

use crate::gameboy::memory::memory_behavior::Memory;
use crate::gameboy::memory::{
    BANK_0_END, BANK_0_START, BANK_1_END, BANK_1_START, ECHO_RAM_END, ECHO_RAM_START, HIGH_RAM_END,
    HIGH_RAM_START, INTERRUPTS_REGISTER, IO_REGISTER_END, IO_REGISTER_START, SPRITE_TABLE_END,
    SPRITE_TABLE_START, VRAM_END, VRAM_START,
};

use super::memory_zone::{
    Bank0, Bank1, EchoRam, ExternalRam, HighRam, InterruptsRegister, IoRegister, ReadOnlyMemory,
    SpriteAttributeTable,
};
use super::{
    vram::VideoRam, BOOT_SEQUENCE_PATH, BOOT_SEQUENCE_SIZE, EXT_RAM_END, EXT_RAM_START, ROM_END,
    ROM_START,
};

// CHECKME
use std::sync::RwLock;

#[derive(Debug, Default)]
/// The memory bus is the shared memory of the GB
/// Components can communicate via this bus
/// TODO use RwLock on each component when getting multithreaded
pub struct MemoryBus {
    /// ROM : boosequence + cartridge game
    read_only_memory: RwLock<ReadOnlyMemory>,
    /// Buffer for display
    video_ram: RwLock<VideoRam>,
    /// RAM brought by the cartridge
    external_ram: RwLock<ExternalRam>,
    /// Work RAM. Does not exist in GB
    bank_0: RwLock<Bank0>,
    /// Work RAM. Presents with GB and CGB
    bank_1: RwLock<Bank1>,
    /// Echo RAM. Usually not used
    echo_ram: RwLock<EchoRam>,
    /// Sprite attribute table
    sprite_attribute_table: RwLock<SpriteAttributeTable>,
    /// IO register to store inputs, sound etc
    io_register: RwLock<IoRegister>,
    /// Yet an other RAM
    high_ram: RwLock<HighRam>,
    /// Interrupt register
    interrupt_register: RwLock<InterruptsRegister>,
}

impl MemoryBus {
    /// Load boot only
    pub fn new() -> Result<Self, String> {
        let memory_bus = MemoryBus::default();
        memory_bus.load_boot()?;
        Ok(memory_bus)
    }

    /// Load boot and cartride
    pub fn load(rom_path: &str) -> Result<Self, String> {
        let memory_bus = Self::new()?;
        memory_bus.load_cartridge(rom_path)?;
        Ok(memory_bus)
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.read().unwrap().read_byte(address),
            VRAM_START..=VRAM_END => self.video_ram.read().unwrap().read_byte(address),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.read().unwrap().read_byte(address),
            BANK_0_START..=BANK_0_END => self.bank_0.read().unwrap().read_byte(address),
            BANK_1_START..=BANK_1_END => self.bank_1.read().unwrap().read_byte(address),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.read().unwrap().read_byte(address),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self
                .sprite_attribute_table
                .read()
                .unwrap()
                .read_byte(address),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.io_register.read().unwrap().read_byte(address)
            }
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.read().unwrap().read_byte(address),
            INTERRUPTS_REGISTER => self.interrupt_register.read().unwrap().read_byte(address),
            _ => panic!("Address {} not handled", address),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        match address {
            ROM_START..=ROM_END => self.read_only_memory.read().unwrap().read_word(address),
            VRAM_START..=VRAM_END => self.video_ram.read().unwrap().read_word(address),
            EXT_RAM_START..=EXT_RAM_END => self.external_ram.read().unwrap().read_word(address),
            BANK_0_START..=BANK_0_END => self.bank_0.read().unwrap().read_word(address),
            BANK_1_START..=BANK_1_END => self.bank_1.read().unwrap().read_word(address),
            ECHO_RAM_START..=ECHO_RAM_END => self.echo_ram.read().unwrap().read_word(address),
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self
                .sprite_attribute_table
                .read()
                .unwrap()
                .read_word(address),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.io_register.read().unwrap().read_word(address)
            }
            HIGH_RAM_START..=HIGH_RAM_END => self.high_ram.read().unwrap().read_word(address),
            INTERRUPTS_REGISTER => self.interrupt_register.read().unwrap().read_word(address),
            _ => panic!("Address {} not handled", address),
        }
    }

    /// return video ram buffer
    /// usefull for GPU
    /// TODO : Terrible atm : this function clone the Vector at *each* frame because
    /// We can't borrow the data inside the RwLockReadGuard
    /// VRAM will likely be moved to the GPU and updated using a channel.
    pub fn vram(&self) -> VideoRam {
        self.video_ram.read().unwrap().clone()
    }

    /// write byte to memory
    pub fn write_byte(&self, address: u16, value: u8) {
        match address {
            ROM_START..=ROM_END => self
                .read_only_memory
                .write()
                .unwrap()
                .write_byte(address, value),
            VRAM_START..=VRAM_END => self.video_ram.write().unwrap().write_byte(address, value),
            EXT_RAM_START..=EXT_RAM_END => self
                .external_ram
                .write()
                .unwrap()
                .write_byte(address, value),
            BANK_0_START..=BANK_0_END => self.bank_0.write().unwrap().write_byte(address, value),
            BANK_1_START..=BANK_1_END => self.bank_1.write().unwrap().write_byte(address, value),
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.echo_ram.write().unwrap().write_byte(address, value)
            }
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self
                .sprite_attribute_table
                .write()
                .unwrap()
                .write_byte(address, value),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.io_register.write().unwrap().write_byte(address, value)
            }
            HIGH_RAM_START..=HIGH_RAM_END => {
                self.high_ram.write().unwrap().write_byte(address, value)
            }
            INTERRUPTS_REGISTER => self
                .interrupt_register
                .write()
                .unwrap()
                .write_byte(address, value),
            _ => panic!("Address {} not handled", address),
        };
    }

    /// write word to memory in the proper subspace
    pub fn write_word(&self, address: u16, value: u16) {
        match address {
            ROM_START..=ROM_END => self
                .read_only_memory
                .write()
                .unwrap()
                .write_word(address, value),
            VRAM_START..=VRAM_END => self.video_ram.write().unwrap().write_word(address, value),
            EXT_RAM_START..=EXT_RAM_END => self
                .external_ram
                .write()
                .unwrap()
                .write_word(address, value),
            BANK_0_START..=BANK_0_END => self.bank_0.write().unwrap().write_word(address, value),
            BANK_1_START..=BANK_1_END => self.bank_1.write().unwrap().write_word(address, value),
            ECHO_RAM_START..=ECHO_RAM_END => {
                self.echo_ram.write().unwrap().write_word(address, value)
            }
            SPRITE_TABLE_START..=SPRITE_TABLE_END => self
                .sprite_attribute_table
                .write()
                .unwrap()
                .write_word(address, value),
            IO_REGISTER_START..=IO_REGISTER_END => {
                self.io_register.write().unwrap().write_word(address, value)
            }
            HIGH_RAM_START..=HIGH_RAM_END => {
                self.high_ram.write().unwrap().write_word(address, value)
            }
            INTERRUPTS_REGISTER => self
                .interrupt_register
                .write()
                .unwrap()
                .write_word(address, value),
            _ => panic!("Address {} not handled", address),
        }
    }

    /// Load boot sequence to the beginning of the ROM
    pub fn load_boot(&self) -> Result<(), String> {
        let memory = &mut self.read_only_memory.write().unwrap();
        let mut boot_sequence = File::open(BOOT_SEQUENCE_PATH).unwrap();

        let boot_size = boot_sequence
            .read(memory.buffer_as_mut())
            .map_err(|e| format!("Failed to parse boot sequence : {}", e))?;
        if boot_size != BOOT_SEQUENCE_SIZE {
            Err("Invalid read size".to_string())
        } else {
            Ok(())
        }
    }

    /// Load cartridge after the boot sequence.
    pub fn load_cartridge(&self, cartrige_path: &str) -> Result<(), String> {
        let memory = &mut self.read_only_memory.write().unwrap();
        let mut cartridge = File::open(cartrige_path).map_err(|e| e.to_string())?;
        // "Skip" the first 0x100 bytes
        cartridge
            .read(&mut memory.buffer_as_mut()[BOOT_SEQUENCE_SIZE..0x0200])
            .map_err(|e| format!("Failed to parse cartride : {}", e))?;

        // write content in memoy after boot sequence size
        cartridge
            .read(&mut memory.buffer_as_mut()[BOOT_SEQUENCE_SIZE..=ROM_END.into()])
            .map_err(|e| format!("Failed to parse cartride : {}", e))?;

        Ok(())
    }
}

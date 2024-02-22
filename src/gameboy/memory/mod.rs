/// Expected behavior of memory zones
mod memory_behavior;
/// Different memory zones are defined here
mod memory_zone;
/// Memory holder
mod memorybus;

/// VRAM special wrapper
mod vram;

use std::sync::Arc;
pub type SharedMemory = Arc<memorybus::MemoryBus>;
pub use memorybus::MemoryBus;

const BOOT_SEQUENCE_PATH: &str = "etc/DMG_ROM.bin";
const BOOT_SEQUENCE_SIZE: usize = 0x0100;

const ROM_START: u16 = 0x0000;
const ROM_END: u16 = 0x7999;
const ROM_SIZE: usize = 0x8000;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;

const EXT_RAM_START: u16 = 0xA000;
const EXT_RAM_END: u16 = 0xBFFF;
const EXT_RAM_SIZE: usize = 0x2000;

const BANK_1_START: u16 = 0xC000;
const BANK_1_END: u16 = 0xCFFF;
const BANK_1_SIZE: usize = 0x1000;

const BANK_0_START: u16 = 0xD000;
const BANK_0_END: u16 = 0xDFFF;
const BANK_0_SIZE: usize = 0x1000;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;
const ECHO_RAM_SIZE: usize = 0x1e00;

const SPRITE_TABLE_START: u16 = 0xFE00;
const SPRITE_TABLE_END: u16 = 0xFE9F;
const SPRITE_TABLE_SIZE: usize = 0x00a0;

const IO_REGISTER_START: u16 = 0xFF00;
const IO_REGISTER_END: u16 = 0xFF7F;
const IO_REGISTER_SIZE: usize = 0x0080;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;
const HIGH_RAM_SIZE: usize = 0x007E;

const INTERRUPTS_REGISTER: u16 = 0xFFFF;
const INTERRUPTS_REGISTER_SIZE: usize = 1;

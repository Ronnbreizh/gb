use super::{BANK_0_END, BANK_0_SIZE, BANK_0_START, BANK_1_END, BANK_1_SIZE, BANK_1_START, ECHO_RAM_END, ECHO_RAM_SIZE, ECHO_RAM_START, EXT_RAM_END, EXT_RAM_SIZE, EXT_RAM_START, HIGH_RAM_END, HIGH_RAM_SIZE, HIGH_RAM_START, INTERRUPTS_REGISTER, INTERRUPTS_REGISTER_SIZE, IO_REGISTER_END, IO_REGISTER_SIZE, IO_REGISTER_START, ROM_END, ROM_SIZE, ROM_START, SPRITE_TABLE_END, SPRITE_TABLE_SIZE, SPRITE_TABLE_START, VRAM_END, VRAM_SIZE, VRAM_START, memory_behavior::Memory};

/// From 0x0000 to 0x7FFF
/// Read only memory : boot sequence + cartridge
#[derive(Debug)]
pub struct ReadOnlyMemory {
    buffer: [u8; ROM_SIZE],
}

impl Default for ReadOnlyMemory {
    fn default() -> Self {
        Self {
            buffer : [0u8; ROM_SIZE]
        }
    }
}

impl Memory for ReadOnlyMemory {
    fn start() -> u16 {
        ROM_START
    }

    fn end() -> u16 {
        ROM_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct VideoRam {
    buffer: [u8; VRAM_SIZE],
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            buffer : [0u8; VRAM_SIZE]
        }
    }
}

impl Memory for VideoRam {
    fn start() -> u16 {
        VRAM_START
    }

    fn end() -> u16 {
        VRAM_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct ExternalRam {
    buffer: [u8; EXT_RAM_SIZE],
}

impl Default for ExternalRam {
    fn default() -> Self {
        Self {
            buffer : [0u8; EXT_RAM_SIZE]
        }
    }
}

impl Memory for ExternalRam {
    fn start() -> u16 {
        EXT_RAM_START
    }

    fn end() -> u16 {
        EXT_RAM_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct Bank1 {
    buffer : [u8; BANK_1_SIZE]
}

impl Default for Bank1 {
    fn default() -> Self {
        Self {
            buffer : [0u8; BANK_1_SIZE]
        }
    }
}

impl Memory for Bank1 {
    fn start() -> u16 {
        BANK_1_START
    }

    fn end() -> u16 {
        BANK_1_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct Bank0 {
    buffer : [u8; BANK_0_SIZE]
}

impl Default for Bank0 {
    fn default() -> Self {
        Self {
            buffer : [0u8; BANK_0_SIZE]
        }
    }
}

impl Memory for Bank0 {
    fn start() -> u16 {
        BANK_0_START
    }

    fn end() -> u16 {
        BANK_0_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct EchoRam {
    buffer : [u8; ECHO_RAM_SIZE]
}

impl Default for EchoRam {
    fn default() -> Self {
        Self {
            buffer : [0u8; ECHO_RAM_SIZE]
        }
    }
}

impl Memory for EchoRam {
    fn start() -> u16 {
        ECHO_RAM_START
    }

    fn end() -> u16 {
        ECHO_RAM_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
       &mut self.buffer 
    }
}

#[derive(Debug)]
pub struct SpriteAttributeTable {
    buffer : [u8; SPRITE_TABLE_SIZE],
}

impl Default for SpriteAttributeTable {
    fn default() -> Self {
        Self {
            buffer : [0u8; SPRITE_TABLE_SIZE]
        }
    }
}

impl Memory for SpriteAttributeTable {
    fn start() -> u16 {
        SPRITE_TABLE_START
    }

    fn end() -> u16 {
        SPRITE_TABLE_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct IoRegister {
    buffer : [u8; IO_REGISTER_SIZE]
}

impl Default for IoRegister {
    fn default() -> Self {
        Self {
            buffer : [0u8; IO_REGISTER_SIZE]
        }
    }
}

impl Memory for IoRegister {
    fn start() -> u16 {
        IO_REGISTER_START
    }

    fn end() -> u16 {
        IO_REGISTER_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct HighRam {
    buffer : [u8; HIGH_RAM_SIZE]
}

impl Default for HighRam {
    fn default() -> Self {
        Self {
            buffer : [0u8; HIGH_RAM_SIZE]
        }
    }
}

impl Memory for HighRam {
    fn start() -> u16 {
        HIGH_RAM_START
    }

    fn end() -> u16 {
        HIGH_RAM_END
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}

#[derive(Debug)]
pub struct InterruptsRegister {
    buffer : [u8; INTERRUPTS_REGISTER_SIZE]
}

impl Default for InterruptsRegister {
    fn default() -> Self {
        Self {
            buffer : [0u8; INTERRUPTS_REGISTER_SIZE]
        }
    }
}

impl Memory for InterruptsRegister {
    fn start() -> u16 {
        INTERRUPTS_REGISTER
    }

    fn end() -> u16 {
        INTERRUPTS_REGISTER
    }

    fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }
}
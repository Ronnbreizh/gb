use super::Gpu;

const LCD_CONTROL_REGISTER_ADDRESS: u16 = 0xFF40;

/// Offset the index to the tilemap to go from 0-255 OR 256 -> 384
pub enum BgWindowDataArea {
    // 0x8000-8FFF - Low index
    Low = 1,
    // 0x8800-97FF - High index
    High = 0,
}

/// Shows which TileMap is in use
#[derive(Debug)]
pub enum TileMap {
    // 9800-9BFF
    One = 0,
    // 9C00-9FFFF
    Two = 1,
}

impl Gpu {
    #[inline]
    fn lcd_control_register(&self) -> u8 {
        self.memory.read_byte(LCD_CONTROL_REGISTER_ADDRESS)
    }

    /// Return true if the LCD screen is enabled
    pub fn _lcd_enabled(&self) -> bool {
        (self.lcd_control_register() >> 7) == 1
    }

    /// Return if the offset inside the TileMap is enabled
    pub fn background_and_windows_tiles(&self) -> BgWindowDataArea {
        if (self.lcd_control_register() << 3 >> 7) == 1 {
            BgWindowDataArea::High
        } else {
            BgWindowDataArea::Low
        }
    }

    /// Return the active TileMap
    pub fn background_tile_map_area(&self) -> TileMap {
        let register = self.lcd_control_register();
        log::trace!("LCD register :{:b}", register);
        if ((register << 4) >> 7) == 0 {
            TileMap::One
        } else {
            TileMap::Two
        }
    }
}

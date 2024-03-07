use super::{memory_behavior::Memory, VRAM_END, VRAM_START};

const TILE_NUMBER: usize = 384;

/// Video Ram, mostly used by the GPU
/// TODO: Remove clone
#[derive(Debug, Clone)]
pub struct VideoRam {
    /// 0x8000 -> 0x97FF
    pub tile_data: [Tile; TILE_NUMBER],
    /// 0x9800 -> 0x9BFF
    pub tile_map_1: [u8; 32 * 32],
    /// 0x9C00 -> 0x9FFF
    pub tile_map_2: [u8; 32 * 32],
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            tile_data: [Tile::default(); 384],
            tile_map_1: [0; 1024],
            tile_map_2: [0; 1024],
        }
    }
}

impl VideoRam {
    /// Return Tile plus the line offset, computed from the address
    fn get_tile_with_line_offset(&mut self, address: u16) -> (&mut Tile, usize) {
        let real_offset = (address - VRAM_START) as usize;
        let quotient = real_offset.rem_euclid(384);
        let remain = real_offset.rem_euclid(8);
        (&mut self.tile_data[quotient], remain)
    }
}

/// Each tile contains 8*8 pixels, each stored on 2 bits.
/// So 16 bits - or 2 bytes - make a line.
/// The color is computer from higher_byte[i]*2 + low_byte[i].
/// The value are therefore splitted to be able to zip them.
#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    pub(crate) higher_bytes: [u8; 8],
    pub(crate) lower_bytes: [u8; 8],
}

impl Tile {
    /// Write the word to the corresponding line in the Tile.
    fn write_line(&mut self, line_offset: usize, value: u16) {
        let [low_value, high_value] = value.to_be_bytes();
        self.higher_bytes[line_offset] = high_value;
        self.lower_bytes[line_offset] = low_value;
    }

    fn write_higher_byte(&mut self, line_offset: usize, value: u8) {
        self.higher_bytes[line_offset] = value;
    }

    fn write_lower_byte(&mut self, line_offset: usize, value: u8) {
        self.lower_bytes[line_offset] = value;
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
        panic!("Do not access this directly")
    }

    fn buffer_as_mut(&mut self) -> &mut [u8] {
        panic!("Do not access this directly")
    }

    fn write_word(&mut self, address: u16, value: u16) {
        match address {
            0x8000..=0x97FF => {
                let (tile, line_offset) = self.get_tile_with_line_offset(address);
                tile.write_line(line_offset, value);
            }
            0x9800..=0x09BFF => {
                todo!()
            }
            0x9C00..=0x9FFF => {
                todo!()
            }
            _ => unimplemented!(),
        }
    }

    /// TODO : check this, writing to the tile seems to not be working as expected
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x97FF => {
                let (tile, line_offset) = self.get_tile_with_line_offset(address);
                if address.div_euclid(2) == 0 {
                    // is even
                    tile.write_lower_byte(line_offset, value);
                } else {
                    // is odd
                    tile.write_higher_byte(line_offset, value);
                }
            }
            0x9800..=0x09BFF => {
                log::debug!("Bind tilemap_1[{}] to tile {}", address - 0x9800, value);
                self.tile_map_1[address as usize - 0x9800] = value;
            }
            0x9C00..=0x9FFF => {
                log::debug!("Bind tilemap_2[{}] to tile {}", address - 0x9800, value);
                self.tile_map_2[address as usize - 0x9C00] = value;
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_tile() {
        let mut tile = Tile::default();
        tile.write_line(0, 0x3C7E);
        assert_eq!(tile.higher_bytes[0], 0x7E);
        assert_eq!(tile.lower_bytes[0], 0x3C);
    }

    #[test]
    fn write_tile_map() {
        let mut vram = VideoRam::default();
        vram.write_byte(0x9C42, 0x42);
        assert_eq!(vram.tile_map_2[0x42], 0x42);
    }
}

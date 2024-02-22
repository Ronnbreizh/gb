use super::{memory_behavior::Memory, VRAM_END, VRAM_START};

const TILE_NUMBER: usize = 384;

/// Video Ram, mostly used by the GPU
#[derive(Debug)]
pub struct VideoRam {
    /// 0x8000 -> 0x97FF
    tile_data: [Tile; TILE_NUMBER],
    /// 0x9800 -> 0x9BFF
    tile_map_1: [u8; 32 * 32],
    /// 0x9C00 -> 0x9FFF
    tile_map_2: [u8; 32 * 32],
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
    fn get_tile_with_line_offset(&mut self, address: u16) -> (&mut Tile, usize) {
        let real_offset = (address - VRAM_START) as usize;
        let quotient = real_offset.div_euclid(TILE_NUMBER);
        let remain = real_offset.rem_euclid(TILE_NUMBER);
        (&mut self.tile_data[quotient], remain)
    }
}

/// Each tile contains 8*8 pixels, each stored on 2 bits
#[derive(Copy, Clone, Debug)]
struct Tile {
    pixels: [[Pixel; 8]; 8],
}

impl Default for Tile {
    fn default() -> Tile {
        Self {
            pixels: [[Pixel::White; 8]; 8],
        }
    }
}

impl Tile {
    fn write_line(&mut self, line_offset: usize, value: u16) {
        let [low_value, high_value] = value.to_be_bytes();
        for (i, pixel) in self.pixels[line_offset].iter_mut().enumerate() {
            *pixel = Pixel::from_bytes(low_value, high_value, i);
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum Pixel {
    Black = 0,
    DarkGrey = 1,
    LightGrey = 2,
    White = 3,
}

impl From<u8> for Pixel {
    fn from(value: u8) -> Pixel {
        match value {
            0 => Pixel::Black,
            1 => Pixel::DarkGrey,
            2 => Pixel::LightGrey,
            3 => Pixel::White,
            _ => panic!("Impossible pixel value"),
        }
    }
}

impl Pixel {
    /// Helper function to convert word + index into a Pixel
    fn from_bytes(low_value: u8, high_value: u8, index: usize) -> Pixel {
        let low_value_bit = (low_value << index) >> 7;
        let high_value_bit = (high_value << index) >> 7;
        let computed_value = (high_value_bit << 1) + low_value_bit;
        Pixel::from(computed_value)
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
    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x97FF => {
                todo!("Only word here")
            }
            0x9800..=0x09BFF => {
                self.tile_map_1[address as usize - 0x9800] = value;
            }
            0x9C00..=0x9FFF => {
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
        assert_eq!(
            tile.pixels[0],
            [
                Pixel::Black,
                Pixel::LightGrey,
                Pixel::White,
                Pixel::White,
                Pixel::White,
                Pixel::White,
                Pixel::LightGrey,
                Pixel::Black
            ]
        )
    }

    #[test]
    fn write_tile_map() {
        let mut vram = VideoRam::default();
        vram.write_byte(0x9C42, 0x42);
        assert_eq!(vram.tile_map_2[0x42], 0x42);
    }
}

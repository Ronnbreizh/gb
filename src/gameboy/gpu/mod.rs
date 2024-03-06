mod lcd_control_register;
mod pixel;

use lcd_control_register::*;
use pixel::Pixel;

use super::memory::SharedMemory;
use crate::gameboy::memory::VideoRam;

use glium::{texture::ClientFormat, Surface};
use glutin::surface::WindowSurface;
/// Screen Width
pub const SCREEN_W: usize = 160;
/// Screen Height
pub const SCREEN_H: usize = 144;

/// objet in charge of display state of the VRAM
/// from $8000 to $97FF
/// sprite table : 8000 to 8FFF
/// sprites attributes: FE00 to FE9F
/// VRAM background map : 9800 to 9BFF
///                    or 9C00 to 9FFF
pub struct Gpu {
    memory: SharedMemory,
    buffer: Vec<u8>,
}

// LCD control | R/W | 0xFF40
struct _LCDControlRegister {
    lcd_enabled: bool,
    window_tile_map_display_select: _WindowMapDisplaySelect,
    window_display_enabled: bool,
    obj_size: _ObjectSize,
    obj_display_enabled: bool,
    bg_display_enabled: bool,
}

/// Select which tile map
enum _WindowMapDisplaySelect {
    // 0x9800-9BFF, or TileMap 1
    Low = 0,
    // 0x9C00-9FFF, or TileMap 2
    High = 1,
}

enum _ObjectSize {
    // 8*8
    Small = 0,
    Big = 1,
}

// LCD status register | R/W | 0xFF41
struct _LCDStatusRegister {
    ly_coincidence_interrupt_enabled: bool,
    mode_2_oam_interrupt_enabled: bool,
    mode_1_vblank_interrupt_enabled: bool,
    mode_0_hblank_interrupt_enabled: bool,
    coincidence_flag: _CoincidenceFlag,
    mode: _ModeFlag,
}

enum _CoincidenceFlag {
    LycLy,
    LycEqualLy,
}

enum _ModeFlag {
    // During H-blank
    HBlank = 0,
    // During V-blank
    VBlank = 1,
    // During searching OAM-RAM
    Searching = 2,
    // During transfering data to LCD driver
    Transfering = 3,
}

const SCY_ADRESS: u16 = 0xFF42;
const SCX_ADRESS: u16 = 0xFF42;

/// Inside the Window f Winit, we will need to create a Vulkan context
impl Gpu {
    pub fn new(memory: SharedMemory) -> Self {
        Self {
            memory,
            // times 3 due to RBG representation of the data
            // Loading all the tilemap at once :<
            buffer: vec![0x0; SCREEN_H * SCREEN_W * 3],
        }
    }

    /// Scroll Y
    pub fn scy(&self) -> u8 {
        self.memory.read_byte(SCY_ADRESS)
    }
    /// set Scroll Y
    pub fn _set_scy(&self, value: u8) {
        self.memory.write_byte(SCY_ADRESS, value);
    }

    /// Scroll X
    pub fn scx(&self) -> u8 {
        self.memory.read_byte(SCX_ADRESS)
    }
    /// set Scroll X
    pub fn _set_scx(&self, value: u8) {
        self.memory.write_byte(SCX_ADRESS, value);
    }

    /// Read the background tilemaps and write them to the buffers
    /// This could partially be moved to shaders
    pub fn read_background(&mut self, vram: &VideoRam) {
        // select right tile map
        let tilemap_index = self.background_tile_map_area();

        let tilemap = match tilemap_index {
            TileMap::One => vram.tile_map_1,
            TileMap::Two => vram.tile_map_2,
        };

        let _offset_index = match self.background_and_windows_tiles() {
            BgWindowDataArea::High => 127,
            BgWindowDataArea::Low => 0,
        };

        // IMPROVE ME : take care of scy and scx
        // for (tile_counter, tile_index) in tilemap.iter().enumerate() {
        //     let corrected_index = tile_index + offset_index;
        //     let tile = vram.tile_data[corrected_index as usize];

        //     // use tile and tile_counter to write to the right place
        //     let row_offset = tile_counter.div_euclid(32);
        //     let col_offset = tile_counter.rem_euclid(32);

        //     self.buffer[((row_offset*32*8 +col_offset*8)* 3) +2] = 255;
        // }

        // Here, I consider that SCY and SCX are always 0
        for tile_col_index in 0..20 {
            for tile_row_index in 0..16 {
                // the 260 simulate the offset, in theory
                let tile_index = tilemap[260 + tile_row_index * 32 + tile_col_index];
                let tile = vram.tile_data[tile_index as usize];

                for (pixel_row, (high, low)) in tile
                    .higher_bytes
                    .iter()
                    .zip(tile.lower_bytes.iter())
                    .enumerate()
                {
                    for pixel_col in 0..8 {
                        let pixel = Pixel::from_bytes(*low, *high, pixel_col);
                        let index = Self::compute_index(
                            tile_row_index,
                            tile_col_index,
                            pixel_row,
                            pixel_col,
                        );
                        // prepare the slice for the color
                        let slice = index * 3..=index * 3 + 2;
                        self.buffer[slice].copy_from_slice(&pixel.to_rgb());
                    }
                }
            }
        }
    }

    #[inline]
    /// Compute the pixel index in the buffer from the pixel coord within a tile
    /// Beware of the conversion from the 32*32 tile space to the 144*160 pixel space
    fn compute_index(
        tile_row: usize,
        tile_col: usize,
        pixel_row: usize,
        pixel_col: usize,
    ) -> usize {
        (tile_row * 8 + pixel_row) * SCREEN_W + 8 * tile_col + pixel_col
    }

    // DRAW THE UPDATED CONTENT TO THE SCREEN
    pub fn draw(
        &mut self,
        display: &glium::Display<WindowSurface>,
        texture: &mut glium::Texture2d,
    ) {
        let _row = self.scy();
        let _col = self.scx();

        let vram = self.memory.vram();
        self.read_background(&vram);

        let rawimage2d = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(&self.buffer),
            width: SCREEN_W as u32,
            height: SCREEN_H as u32,
            format: ClientFormat::U8U8U8,
        };

        // CHECK is this is wrapping
        texture.write(
            glium::Rect {
                left: 0,
                bottom: 0,
                width: SCREEN_W as u32,
                height: SCREEN_H as u32,
            },
            rawimage2d,
        );

        let target = display.draw();

        // draw
        let (target_w, target_h) = target.get_dimensions();
        let interpolation_type = glium::uniforms::MagnifySamplerFilter::Linear;

        texture.as_surface().blit_whole_color_to(
            &target,
            &glium::BlitTarget {
                left: 0,
                bottom: target_h,
                width: target_w as i32,
                height: -(target_h as i32),
            },
            interpolation_type,
        );
        // finish
        target.finish().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_index() {
        // assert_eq!(Gpu::compute_index(0,0,0,0), 0);
        // assert_eq!(Gpu::compute_index(0,0,7,7), 1799);
        // assert_eq!(Gpu::compute_index(15,19,7,7), 23_903);
    }
}

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
const LCDY_ADRESS: u16 = 0xFF44;

/// Inside the Window f Winit, we will need to create a Vulkan context
impl Gpu {
    pub fn new(memory: SharedMemory) -> Self {
        Self {
            memory,
            // times 3 due to RBG representation of the data
            buffer: vec![0xff; SCREEN_H * SCREEN_W * 3],
        }
    }

    /// Scroll Y
    pub fn scy(&self) -> u8 {
        self.memory.read_byte(SCY_ADRESS)
    }

    /// Scroll X
    pub fn scx(&self) -> u8 {
        self.memory.read_byte(SCX_ADRESS)
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

        let offset_index = match self.background_and_windows_tiles() {
            BgWindowDataArea::High => 128,
            BgWindowDataArea::Low => 0,
        };

        let scy = self.scy() as usize;
        let scx = self.scx() as usize;

        // This should be optimized in a shader
        for pixel_row in 0..SCREEN_H {
            self.memory.write_byte(LCDY_ADRESS, pixel_row as u8);
            for pixel_col in 0..SCREEN_W {
                //  make the value wrap when out of bound due to SCX / SCY
                let tilemap_pixel_row = (pixel_row + scy) % 255;
                let tilemap_pixel_col = (pixel_col + scx) % 255;

                // Get the matching tile ...
                let tilemap_index =
                    Self::convert_pixel_to_tile_coord(tilemap_pixel_row, tilemap_pixel_col);
                let tile_index = tilemap[tilemap_index];
                let tile = vram.tile_data[tile_index as usize + offset_index];
                // ... and get the pixel within this tile.
                let tile_pixel_row = tilemap_pixel_row.rem_euclid(8);
                let tile_pixel_col = tilemap_pixel_col.rem_euclid(8);

                // Compute pixel color
                let pixel = Pixel::from_bytes(
                    tile.lower_bytes[tile_pixel_row],
                    tile.higher_bytes[tile_pixel_row],
                    tile_pixel_col,
                );

                // Write the color
                let index = (pixel_row * SCREEN_W + pixel_col) * 3;
                let slice = index..=index + 2;
                self.buffer[slice].copy_from_slice(&pixel.to_rgb());
            }
        }

        self.memory.write_byte(LCDY_ADRESS, 144);
    }

    /// Convert tilemap row and col - including SCY and SCX - to tileindex and offset within the
    /// corresponding tile.
    fn convert_pixel_to_tile_coord(tilemap_pixel_row: usize, tilemap_pixel_col: usize) -> usize {
        tilemap_pixel_row.div_euclid(8) * 32 + tilemap_pixel_col.div_euclid(8)
    }

    // DRAW THE UPDATED CONTENT TO THE SCREEN
    pub fn draw(
        &mut self,
        display: &glium::Display<WindowSurface>,
        texture: &mut glium::Texture2d,
    ) {
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
    use super::*;
    #[test]
    fn convert_pixel_to_tile_coord() {
        assert_eq!(Gpu::convert_pixel_to_tile_coord(0, 0), 0);
        assert_eq!(Gpu::convert_pixel_to_tile_coord(0, 32), 4);
        assert_eq!(Gpu::convert_pixel_to_tile_coord(9, 0), 32);
        assert_eq!(Gpu::convert_pixel_to_tile_coord(9, 8), 33);
        assert_eq!(Gpu::convert_pixel_to_tile_coord(255, 255), 1023);
    }
}

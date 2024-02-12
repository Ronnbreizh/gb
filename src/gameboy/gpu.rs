//use super::{SCREEN_W, SCREEN_H};
use super::memory::SharedMemory;

use glium::{Display, Surface};
use glutin::surface::WindowSurface;

/// objet in charge of display state of the VRAM
/// from $8000 to $97FF
/// sprite table : 8000 to 8FFF
/// sprites attributes: FE00 to FE9F
/// VRAM background map : 9800 to 9BFF
///                    or 9C00 to 9FFF
pub struct Gpu {
    display: Option<Display<WindowSurface>>,
    memory: SharedMemory,
}

// LCD control | R/W | 0xFF40
struct _LCDControlRegister {
    lcd_enabled: bool,
    window_tile_map_display_select: _WindowMapDisplaySelect,
    window_display_enabled: bool,
    tile_data_select: _TileDataSelect,
    bg_map_display_select: _BgMapDisplaySelect,
    obj_size: _ObjectSize,
    obj_display_enabled: bool,
    bg_display_enabled: bool,
}

enum _WindowMapDisplaySelect {
    // 0x9800-9BFF
    Low = 0,
    // 0x9C00-9FFF
    High = 1,
}

enum _TileDataSelect {
    // 0x8000-8FFF
    Low = 1,
    // 0x8800-97FF
    High = 0,
}

enum _BgMapDisplaySelect {
    // 9800-9BFF
    Low = 0,
    // 9C00-9FFFF
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
            display: None,
            memory,
        }
    }

    pub fn set_display(&mut self, display: Display<WindowSurface>) {
        self.display = Some(display);
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

    // DRAW THE UPDATED CONTENT TO THE SCREEN
    pub fn draw(&self) {
        let _raw = self.scy();
        let _col = self.scx();
        // TODO
        // GLIUM part
        let mut target = self.display.as_ref().expect("No display available").draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();
        /*
        let texture = glium::texture::texture2d::Texture2d::empty_with_format(
                display,
                UncompressedFloatFormat::U8U8U8,
                MipmapsOption::NoMipmap,
                SCREEN_W as u32,
                SCREEN_H as u32)
            .unwrap();

        // Retrive VRAM
        let data = bus.vram();

        let rawimage2d = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(data),
            width: SCREEN_W as u32,
            height: SCREEN_H as u32,
            format: ClientFormat::U8U8U8,
        };

        texture.write(
            glium::Rect {
                left: 0,
                bottom: 0,
                width: SCREEN_W as u32,
                height: SCREEN_H as u32
            },
            rawimage2d);

        let mut target = display.draw();
        // clear
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // draw
        let (target_w,target_h) = target.get_dimensions();
        let interpolation_type = glium::uniforms::MagnifySamplerFilter::Linear;

        texture.as_surface().blit_whole_color_to(
            &target,
            &glium::BlitTarget {
                left: 0,
                bottom: target_h,
                width: target_w as i32,
                height: -(target_h as i32)
            },
            interpolation_type);
        // finish
        target.finish().unwrap();

        */
    }
}

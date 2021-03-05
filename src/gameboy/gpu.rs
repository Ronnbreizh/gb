use super::{SCREEN_W, SCREEN_H};
use super::memorybus::MemoryBus;
use std::rc::Rc;
use std::cell::RefCell;
use glium::{
    Display,
    Surface,
    texture::{ UncompressedFloatFormat,
        MipmapsOption,
        ClientFormat,
    },
};

/// objet in charge of display state of the VRAM
/// from $8000 to $97FF
/// sprite table : 8000 to 8FFF
/// sprites attributes: FE00 to FE9F
/// VRAM background map : 9800 to 9BFF
///                    or 9C00 to 9FFF
pub struct Gpu {
}

impl Gpu {
    pub fn new() -> Self {

        Self {
        }
    }

    pub fn draw(&self, display: &Display, bus: &mut MemoryBus) {
        let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
                display,
                UncompressedFloatFormat::U8U8U8,
                MipmapsOption::NoMipmap,
                SCREEN_W as u32,
                SCREEN_H as u32)
            .unwrap();

        // TODO
        let data = bus.vram();

        let rawimage2d = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(&data),
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
    }
}

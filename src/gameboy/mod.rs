mod arithmetictarget;
mod cpu;
mod flagsregister;
mod gpu;
mod instruction;
mod memory;
mod registers;

use cpu::Cpu;
use gpu::Gpu;
use memory::MemoryBus;
use std::sync::Arc;

/// Screen Width
const _SCREEN_W: u32 = 160;
/// Screen Height
const _SCREEN_H: u32 = 144;

/// Custom result type, for internal purpose mostly
type GbResult<T> = Result<T, String>;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};

/// Gameboy main structure
/// load the rom and play it after boot sequence
/// The bus is a shared buffer between component
/// * audio
/// * cpu
/// * gpu
/// * inputs
pub struct Gameboy {
    cpu: Cpu,
    gpu: Gpu,
}

impl Default for Gameboy {
    fn default() -> Self {
        Self::new()
    }
}

impl Gameboy {
    pub fn new() -> Self {
        let bus = Arc::new(MemoryBus::default());
        Self {
            cpu: Cpu::new(bus.clone()),
            gpu: Gpu::new(bus.clone()),
        }
    }

    pub fn load(rom_path: &str) -> GbResult<Self> {
        let bus = Arc::new(MemoryBus::load(rom_path)?);
        Ok(Self {
            cpu: Cpu::new(bus.clone()),
            gpu: Gpu::new(bus.clone()),
        })
    }

    pub fn run(mut self) {
        println!("Run");
        let event_loop = EventLoopBuilder::new()
            .build()
            .expect("Failed to build event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

        self.gpu.set_display(display);
        let _res = event_loop.run(move |ev, window_target| {
            // cpu
            self.cpu.step();

            // video
            self.gpu.draw();
            // audio

            // compute frame frequency?
            // let _next_frame_time = std::time::Instant::now() +
            //     std::time::Duration::from_secs(2);

            // input
            match ev {
                Event::WindowEvent { event, .. } => {
                    if event == WindowEvent::CloseRequested {
                        window_target.exit();
                    }
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        });
    }
}

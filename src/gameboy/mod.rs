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

/// Screen Width
const _SCREEN_W: u32 = 160;
/// Screen Height
const _SCREEN_H: u32 = 144;

/// Custom result type, for internal purpose mostly
type GbResult<T> = Result<T, String>;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
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
    bus: MemoryBus,
    gpu: Gpu,
}

impl Default for Gameboy {
    fn default() -> Self {
        Self::new()
    }
}

impl Gameboy {
    pub fn new() -> Self {
        let bus = MemoryBus::default();
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            bus,
        }
    }

    pub fn load(rom_path: &str) -> GbResult<Self> {
        let bus = MemoryBus::load(rom_path)?;
        Ok(Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            bus,
        })
    }

    pub fn run(mut self) {
        println!("Run");
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop
            .run(move |ev, control_flow| {
                // cpu
                self.cpu.step(&mut self.bus);

                // video
                self.gpu.draw(&mut self.bus);
                // audio

                // compute frame frequency?
                // let _next_frame_time = std::time::Instant::now() +
                //     std::time::Duration::from_secs(2);

                // input
                match ev {
                    Event::WindowEvent { event, .. } => {
                        if event == WindowEvent::CloseRequested {
                            control_flow.exit();
                        }
                    }
                    Event::AboutToWait => {
                        window.request_redraw();
                    }
                    _ => (),
                }
            })
            .unwrap();
    }
}

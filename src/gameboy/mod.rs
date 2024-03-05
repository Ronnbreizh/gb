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
use std::time::Duration;

use glium::{
    texture::{MipmapsOption, UncompressedFloatFormat},
    Texture2d,
};

// 4MHz frequency - or 8MHz in CGB double frequency mode.
const CPU_TICK_DURATION: std::time::Duration = Duration::from_nanos(250);

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

    pub fn run(self) {
        println!("Run");

        let Gameboy { mut cpu, mut gpu } = self;

        let event_loop = EventLoopBuilder::new()
            .build()
            .expect("Failed to build event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

        let mut texture = Texture2d::empty_with_format(
            &display,
            UncompressedFloatFormat::U8U8U8,
            MipmapsOption::NoMipmap,
            gpu::SCREEN_W as u32,
            gpu::SCREEN_H as u32,
        )
        .expect("Failed to create texture");

        let _cpu_thread_handle = std::thread::Builder::new()
            .name("CPU-thread".to_string())
            .spawn(move || {
                // CPU Loop
                '_cpu_loop: loop {
                    // We neglate the delta time took by the emulator CPU given the factor 1000 between
                    // each.
                    let cpu_delay_cycles = cpu.step();
                    // Compute delay
                    std::thread::sleep(cpu_delay_cycles * CPU_TICK_DURATION);
                }
            });

        let _res = event_loop.run(move |ev, window_target| {
            // video
            gpu.draw(&display, &mut texture);
            // audio

            // compute frame frequency?
            // let _next_frame_time = std::time::Instant::now() +
            //     std::time::Duration::from_secs(2);

            // TODO : input event should write to registers
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

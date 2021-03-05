use super::cpu::Cpu;
use super::memorybus::MemoryBus;
use super::gpu::Gpu;
use std::rc::Rc;
use std::cell::RefCell;
use glium::{
    glutin::{
        event_loop::{
            EventLoop,
            ControlFlow,
        },
        event::{
            Event,
            WindowEvent,
        },
        ContextBuilder,
        window::WindowBuilder,
    },
    Display,
    Surface,
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
    bus: Rc<RefCell<MemoryBus>>,
    gpu: Gpu,
}

impl Gameboy {
    pub fn new() -> Self {
        let bus = MemoryBus::new();
        Self{
            cpu : Cpu::new(),
            gpu : Gpu::new(),
            bus,
        }
    }

    pub fn load(rom_path: &str) -> Self {
        let bus = MemoryBus::load(rom_path);
        Self{
            cpu : Cpu::new(),
            gpu : Gpu::new(),
            bus,
        }
    }

    pub fn run(mut self) {
        println!("Run");
        let event_loop = EventLoop::new();

        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        event_loop.run(move |ev, _, control_flow| {

            // cpu
            self.cpu.step(&mut self.bus);

            // video
            self.gpu.draw(&display);

            // audio

            // compute frame frequency?
            let next_frame_time = std::time::Instant::now() +
                std::time::Duration::from_nanos(16_666_667);

            *control_flow = ControlFlow::WaitUntil(next_frame_time);

            // input
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }
}

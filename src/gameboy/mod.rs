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

use winit::{
    self,
    application::ApplicationHandler,
    event::{self, ElementState, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    platform::{wayland::EventLoopBuilderExtWayland, x11::EventLoopBuilderExtX11},
    raw_window_handle::HasDisplayHandle,
    window::{Window, WindowAttributes},
};

// 4MHz frequency - or 8MHz in CGB double frequency mode.
const CPU_TICK_DURATION: std::time::Duration = Duration::from_nanos(250);

/// Custom result type, for internal purpose mostly
type GbResult<T> = Result<T, String>;

/// Gameboy main structure
/// load the rom and play it after boot sequence
/// The bus is a shared buffer between component
/// * audio
/// * cpu
/// * gpu
/// * inputs
pub struct Gameboy {
    bus: Arc<MemoryBus>,
}

impl Gameboy {
    pub fn new() -> GbResult<Self> {
        Ok(Self::new_inner(MemoryBus::new()?))
    }

    pub fn load(rom_path: &str) -> GbResult<Self> {
        Ok(Self::new_inner(MemoryBus::load(rom_path)?))
    }

    fn new_inner(memory: MemoryBus) -> Self {
        let shared_memory = Arc::new(memory);
        Self { bus: shared_memory }
    }

    fn spawn_cpu(&self) {
        let mut cpu = Cpu::new(self.bus.clone());

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
    }

    pub fn run(self) {
        let event_loop = EventLoop::builder()
            .build()
            .expect("Failed to build event loop");

        event_loop.set_control_flow(ControlFlow::Poll);
        self.spawn_cpu();
        let Gameboy { bus } = self;

        let gpu = Gpu::new(bus.clone());

        let mut app_context = AppContext {
            _bus: bus,
            window: None,
            gpu,
        };

        // Main loop, graphical and inputs
        let _res = event_loop.run_app(&mut app_context);
    }
}

struct AppContext {
    _bus: Arc<MemoryBus>,
    gpu: Gpu,
    window: Option<Window>,
}

impl Drop for AppContext {
    fn drop(&mut self) {
        log::info!("Dropping app context");
    }
}

impl ApplicationHandler for AppContext {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Gamebouille")
            .with_visible(true);
        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to build window");
        self.window = Some(window);
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::Key(_raw_key_event) => todo!(),
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.gpu.draw();
                window.request_redraw();
            }
            _ => (),
        }
    }
}

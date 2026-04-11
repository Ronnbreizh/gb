use std::env::{self, home_dir};
mod logging;
use gb::Gameboy;

fn main() {
    // Workaroud for WSL
    std::env::set_var("WAYLAND_DISPLAY", ""); // Force X11 on Linux
    std::env::set_var("WGPU_BACKEND", "gl"); // Force X11 on Linux

    // TODO : clap for cleaner CLI
    let filename = env::args().nth(1);
    let gameboy = if let Some(filename) = filename {
        // load ROM
        log::info!("Run with ROM {}", filename);
        Gameboy::load(&filename).unwrap()
    } else {
        // Only the Bootstrap
        log::info!("Run without ROM");
        Gameboy::new().unwrap()
    };

    // gameboy.run()
    gb::hmi::run();
}

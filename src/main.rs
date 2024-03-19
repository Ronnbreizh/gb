use std::env;
mod logging;
use gb::Gameboy;

fn main() {
    // Init logging
    simple_logging::log_to_file("test.log", logging::log_level())
        .expect("Failed to create logging env");
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

    gameboy.run()
}

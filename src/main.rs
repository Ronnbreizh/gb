use std::env;

use gb::Gameboy;

fn main() {
    // Init logging
    simple_logging::log_to_file("test.log", log::LevelFilter::Debug)
        .expect("Failed to create logging env");
    let filename = env::args().nth(1);

    let gameboy = if let Some(filename) = filename {
        // load ROM
        Gameboy::load(&filename).unwrap()
    } else {
        // Only the Bootstrap
        Gameboy::new().unwrap()
    };

    gameboy.run()
}

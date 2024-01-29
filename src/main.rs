use std::env;

use gb::Gameboy;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args.first().expect("No ROM path provided");

    // load ROM
    let gameboy = Gameboy::load(filename).unwrap();

    gameboy.run()
}

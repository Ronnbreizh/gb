use gb::Gameboy;

fn main() {
    // load ROM
    let gameboy = Gameboy::new();

    gameboy.run();
}

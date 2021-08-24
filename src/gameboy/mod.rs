mod gameboy;
mod registers;
mod flagsregister;
mod instruction;
mod memory;
mod arithmetictarget;
mod cpu;
mod gpu;

pub use gameboy::Gameboy;

const SCREEN_W : u32 = 160;
const SCREEN_H : u32 = 144;

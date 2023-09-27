pub mod peripherals;
pub mod emulator;
pub mod utils;

use emulator::Emulator;

fn main() {
    let mut emulator = Emulator::new();
    emulator.load_rom("/Users/danterendell/Documents/Personal/Dev/Rust/chip8_emu/roms/IBM Logo.ch8");
    emulator.run();
}


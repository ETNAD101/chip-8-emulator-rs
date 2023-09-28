pub mod peripherals;
pub mod emulator;
pub mod utils;

use std::env;
use emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom;
    match args.len() {
        1 => rom = "roms/IBM Logo.ch8",
        2 => rom = &args[1],
        _ => panic!("Too many arguments provided")
    }
    let mut emulator = Emulator::new();
    emulator.load_rom(rom);
    emulator.run();
}


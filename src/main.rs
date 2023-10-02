/*
TODOS
1. Add visual debugger
2. Fix broken opcodes
3. Cleanup the code
4. Work on better error handling and use options & results
5. fix speed from printing to screen
*/

pub mod peripherals;
pub mod emulator;
pub mod utils;

use std::env;
use emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rom;
    match args.len() {
        1 => rom = "roms/Chip8 emulator Logo [Garstyciuks].ch8",
        2 => rom = &args[1],
        _ => panic!("Too many arguments provided")
    }
    let mut emulator = Emulator::new();
    emulator.load_rom(rom);
    emulator.run();
}


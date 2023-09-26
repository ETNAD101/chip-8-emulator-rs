pub mod peripherals;
pub mod emulator;

use emulator::Emulator;
use peripherals::display::Display;

fn main() {
    let mut emulator = Emulator::new();
    emulator.run();

    let mut display = Display::new();
    display.run(); 
}


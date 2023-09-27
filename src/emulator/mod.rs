use std::time::Duration;
use std::fs;

use crate::peripherals::display::Display;
use crate::utils::OFstate;

pub const CLOCK_SPEED: u32 = 700;

pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;

pub const PRGRM_START: usize = 0x200;

pub const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Emulator {
    display: Display,
    memory: [u8; MEM_SIZE],
    stack: [u16; STACK_SIZE], 
    stack_pointer: usize,
    program_counter: u16,
    register_i: u16,
    register_v: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new(),
            memory: [0; MEM_SIZE],
            stack: [0; STACK_SIZE],
            stack_pointer: PRGRM_START,
            program_counter: 0,
            register_i: 0,
            register_v: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    
    pub fn run(&mut self) { 
        self.load_font();
        self.dump_mem();

        loop {
            let opcode = self.fetch();
            self.execute(opcode);
            self.display.run();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / CLOCK_SPEED));
        }
    }

    pub fn dump_mem(&self) {
        for i in 0..MEM_SIZE {
            if i % 32 == 0 {
                println!();
            }
            if i % 16 == 0 {
                print!("| {:#05x} | ", i);
            }
            
            print!(" {:02x} ", self.memory[i]);
        }
        println!();
    }

    pub fn load_rom(&mut self, path: &str) {
        let rom = fs::read(path).unwrap();

        for i in 0..rom.len() {
            self.memory[PRGRM_START + i] = rom[i];
        };
    }

    fn load_font(&mut self) {
        for i in 0..FONT.len() {
            self.memory[0x50 + i] = FONT[i];
        };
    }


    fn push_stack(&mut self, data: u16) {
        self.stack[self.stack_pointer] = data;
        self.stack_pointer += 1;
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.stack[self.stack_pointer];
        self.stack[self.stack_pointer] = 0;
        self.stack_pointer -= 1;
        data
    }

    fn fetch(&mut self) -> (u8, u8, u8, u8) {
        let first_byte = self.memory[self.stack_pointer] as u16;
        let second_byte = self.memory[self.stack_pointer + 1] as u16;
        let instruction = first_byte << 8 | second_byte;
        self.program_counter += 2;
        
        (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        )
    }

    fn execute(&mut self, opcode: (u8, u8, u8, u8)) {
        let x = opcode.1;
        let y = opcode.2;
        let single_n = opcode.3;
        let double_n = (opcode.2 << 4) | opcode.3 ;
        let triple_n = ((opcode.1 as u16) << 8) | ((opcode.2 as u16) << 4) | opcode.3 as u16;

        match opcode {
            (0x0, 0x0, 0xe, 0x0) => self.display.clear(),
            (0x1, _, _, _) => self.jump(triple_n),
            (0x6, _, _, _) => self.set_vx(x, double_n),
            (0xa, _, _, _) => self.set_register_i(triple_n),
            (0xd, _, _, _) => self.disp(x, y, single_n),
            (_, _, _, _) => panic!("Unknown code")
        }
    }

    fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn set_vx(&mut self, register: u8, value: u8) {
        self.register_v[register as usize] = value;
    }
    
    fn add_vx(&mut self, register: u8, value: u8) {
        self.register_v[register as usize] += value;
    }

    fn set_register_i(&mut self, value: u16) {
        self.register_i = value;
    }

    fn disp(&mut self, x: u8, y: u8, n: u8) {
        let mut x_coord = self.register_v[x as usize] % 64;
        let mut y_coord = self.register_v[y as usize] % 32;
        self.register_v[0xf] = 0;
        for offset in 0..n {
            let byte = self.memory[(self.register_i + offset as u16) as usize];
            for i in 0..8 {
                if (byte >> i & 1) == 1 {
                    if self.display.pixel_state[x_coord as usize][y_coord as usize].state == OFstate::ON {
                        self.display.toggle_pixel(x_coord, y_coord, OFstate::OFF);
                        self.register_v[0xF] = 1;
                    } else {
                        self.display.toggle_pixel(x_coord, y_coord, OFstate::ON);
                    }
                }
                if x_coord > 63 {
                    break;
                }
                x_coord += 1;
            }
            y_coord += 1;
            if y_coord > 31 {
                break;
            }
        }
    }
}




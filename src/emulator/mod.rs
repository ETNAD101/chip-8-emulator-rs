use std::time::Duration;
use std::fs;

use crate::peripherals::display::Display;
use crate::utils::OFstate;

use crate::utils::X_PIXELS;
use crate::utils::Y_PIXELS;

pub const CLOCK_SPEED: u32 = 700;

pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;

pub const PRGRM_START: u16 = 0x200;

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
    vram: [[OFstate; 64];32],
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
            vram: [[OFstate::OFF; 64]; 32],
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            program_counter: PRGRM_START,
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
            self.display.run(self.vram);
            self.execute(opcode);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / CLOCK_SPEED));
        }
    }

    pub fn dump_mem(&self) {
        let mut x = 0;
        for i in 0..32 {
            if i % 16 == 0 {
                print!("|memAddr| ");
                x = 0;
            }
            print!(" {:02x} ", x);
            x += 1;
        }
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
            self.memory[PRGRM_START as usize + i] = rom[i];
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
        let first_byte = self.memory[self.program_counter as usize] as u16;
        let second_byte = self.memory[self.program_counter as usize + 1] as u16;
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
            (0x00, 0x00, 0x0e, 0x00) => self.display.clear(),
            (0x00, 0x00, 0x0e, 0x0e) => self.sub_return(),
            (0x05, _, _, 0x00) => self.skip_both_equal(x, y),
            (0x08, _, _, 0x00) => self.set_vx_vy(x, y),
            (0x08, _, _, 0x01) => self.binary_or(x, y),
            (0x08, _, _, 0x02) => self.binary_and(x, y),
            (0x08, _, _, 0x03) => self.logical_xor(x, y),
            (0x08, _, _, 0x04) => self.add_vx_vy(x, y),
            (0x08, _, _, 0x05) => self.vx_minus_vy(x, y),
            (0x08, _, _, 0x06) => self.shift_right(x),
            (0x08, _, _, 0x07) => self.vy_minus_vx(x, y),
            (0x08, _, _, 0x0e) => self.shift_left(x),
            (0x09, _, _, 0x00) => self.skip_none_equal(x, y),
            (0x01, _, _, _) => self.jump(triple_n),
            (0x02, _, _, _) => self.subroutine(triple_n),
            (0x03, _, _, _) => self.skip_if_equal(x, double_n),
            (0x04, _, _, _) => self.skip_not_equal(x, double_n),
            (0x06, _, _, _) => self.set_vx(x, double_n),
            (0x07, _, _, _) => self.add_vx(x, double_n),
            (0x0a, _, _, _) => self.set_register_i(triple_n),
            (0x0d, _, _, _) => self.disp(x, y, single_n),
            (_, _, _, _) => panic!("Unknown opcode at: {:#02x}", self.program_counter)
        }
    }

    fn subroutine(&mut self, nnn: u16) {
        self.push_stack(self.program_counter);
        self.program_counter = nnn;
    }

    fn sub_return(&mut self) {
        self.program_counter = self.pop_stack();
    }

    fn jump(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    fn binary_or(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx | vy;
    }

    fn binary_and(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx & vy;
    }

    fn logical_xor(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx ^ vy;
    }

    fn set_vx(&mut self, x: u8, nn: u8) {
        self.register_v[x as usize] = nn;
    }

    fn set_vx_vy(&mut self, x: u8, y: u8) {
        self.register_v[x as usize] = self.register_v[y as usize];
    }
    
    fn add_vx(&mut self, x: u8, nn: u8) {
        self.register_v[x as usize] += nn;
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize] as u16;
        let vy = self.register_v[y as usize] as u16; 
        let result = vx + vy;
        self.register_v[x as usize] = result as u8;

        self.register_v[0x0f] = if result > 255 {1} else {0};
    }

    fn vx_minus_vy(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        let result = vx - vy; 
        self.register_v[x as usize] = result;

        self.register_v[0x0f] = if vx > vy {1} else {0};
    }

    fn vy_minus_vx(&mut self, x: u8, y: u8) {
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        let result = vy - vx; 
        self.register_v[x as usize] = result;

        self.register_v[0x0f] = if vy > vx {1} else {0};
    }

    fn shift_left(&mut self, x: u8) {
        self.register_v[0x0f] = self.register_v[x as usize] & 1;
        self.register_v[x as usize] >>= 1;
    }

    fn shift_right(&mut self, x: u8) {
        self.register_v[0x0f] = (self.register_v[x as usize] & 0b10000000) >> 7;
        self.register_v[x as usize] <<= 1;
    }

    fn set_register_i(&mut self, value: u16) {
        self.register_i = value;
    }

    fn skip_if_equal(&mut self, x: u8, nn: u8) {
        if self.register_v[x as usize] == nn {
            self.program_counter += 2;
        }
    }

    fn skip_not_equal(&mut self, x: u8, nn: u8) {
        if self.register_v[x as usize] != nn {
            self.program_counter += 2;
        }
    }

    fn skip_both_equal(&mut self, x: u8, y:u8) {
        if self.register_v[x as usize] == self.register_v[y as usize] {
            self.program_counter += 2;
        }
    }

    fn skip_none_equal(&mut self, x: u8, y:u8) {
        if self.register_v[x as usize] != self.register_v[y as usize] {
            self.program_counter += 2;
        }
    }

    fn disp(&mut self, x: u8, y: u8, n: u8) {
        self.register_v[0x0f] = 0;
        for offset in 0..n {
            let y_coord = (self.register_v[y as usize] + offset) % Y_PIXELS as u8;

            for bit in 0..8 {
                let x_coord = (self.register_v[x as usize] + bit) % X_PIXELS as u8;
                let byte = (self.memory[(self.register_i + offset as u16) as usize] >> (7-bit)) & 1;

                if byte == 1 {
                    if self.vram[y_coord as usize][x_coord as usize] == OFstate::ON {
                        self.vram[y_coord as usize][x_coord as usize] = OFstate::OFF;
                        self.register_v[0x0F] = 1;
                    } else {
                        self.vram[y_coord as usize][x_coord as usize] = OFstate::ON;
                    }
                }
                if x_coord > 63 {
                    break;
                }
            }
            if y_coord > 31 {
                break;
            }
        }        
    }
}




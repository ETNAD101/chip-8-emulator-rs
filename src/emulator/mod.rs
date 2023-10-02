use std::time::Duration;
use std::fs;
use rand;

use crate::peripherals::display::Display;
use crate::peripherals::display::PeripheralMemory;
use crate::peripherals::display::TextData;
use crate::utils::OFstate;

use crate::utils::TextType;
use crate::utils::X_PIXELS;
use crate::utils::Y_PIXELS;

pub const CLOCK_SPEED: u32 = 720;

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
    p_mem: PeripheralMemory,
    stack: [u16; STACK_SIZE], 
    stack_pointer: usize,
    program_counter: u16,
    register_i: u16,
    register_v: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    time_count: u8,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            display: Display::new(),
            memory: [0; MEM_SIZE],
            p_mem: PeripheralMemory::new([[OFstate::OFF; 64]; 32], [false; 16], Vec::new()),
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            program_counter: PRGRM_START,
            register_i: 0,
            register_v: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            time_count: 0,
        }
    }
    
    pub fn run(&mut self) { 
        self.load_font();
        self.dump_mem();

        loop {
            self.time_count += 1;
            if self.time_count == 12 {
                self.decrement_timers();
                self.time_count = 0;
            }
            let opcode = self.fetch();
            self.display.run(&mut self.p_mem);
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

    pub fn dump_stack(&self) {
        println!("Stack:");
        for i in 0..STACK_SIZE {
            println!("{:#05x}", self.stack[i])
        }
        println!()
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
        self.stack_pointer -= 1;
        let data = self.stack[self.stack_pointer];
        println!("stack return: {:#05x}", data);
        self.stack[self.stack_pointer] = 0;
        data
    }

    fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
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
        println!("{:#05x}", self.program_counter - 2);
        match opcode {
            (0x00, 0x00, 0x0e, 0x00) => self.display.clear(),
            (0x00, 0x00, 0x0e, 0x0e) => self.sub_return(),
            (0x0e, _, 0x09, 0x0e) => self.skip_if_key(x),
            (0x0e, _, 0x0a, 0x01) => self.skip_not_key(x),
            (0x0f, _, 0x00, 0x07) => self.set_vx_to_delay(x),
            (0x0f, _, 0x00, 0x0a) => self.get_key(x),
            (0x0f, _, 0x01, 0x05) => self.set_delay_timer(x),
            (0x0f, _, 0x01, 0x08) => self.set_sound_timer(x),
            (0x0f, _, 0x01, 0x0e) => self.add_to_index(x),
            (0x0f, _, 0x02, 0x09) => self.font_char(x),
            (0x0f, _, 0x03, 0x03) => self.bcd_convert(x),
            (0x0f, _, 0x05, 0x05) => self.store_mem(x),
            (0x0f, _, 0x06, 0x05) => self.load_mem(x),
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
            (0x0a, _, _, _) => self.set_index(triple_n),
            (0x0b, _, _, _) => self.offset_jump(triple_n),
            (0x0c, _, _, _) => self.random(x, double_n),
            (0x0d, _, _, _) => self.disp(x, y, single_n),
            (_, _, _, _) => panic!("Unknown opcode at {:#05x}: ({:x}, {:x}, {:x}, {:x})", self.program_counter - 2, opcode.0, opcode.1, opcode.2, opcode.3)
        }
        self.dump_stack();
    }

    fn get_key(&mut self, x: u8) {
        let msg = format!("Get_key called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        for i in 0..self.p_mem.key_list.len() {
            if self.p_mem.key_list[i] {
                self.register_v[x as usize] = i as u8;
                return;
            } 
        }
        self.program_counter -= 2;
    }

    fn store_mem(&mut self, x: u8) {
        let msg = format!("store_mem called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        for i in 0..=x {
            let value = self.register_v[i as usize];
            let pos = (self.register_i + i as u16) as usize;
            self.memory[pos] = value;
        }
    }

    fn load_mem(&mut self, x: u8) {
        let msg = format!("store_mem called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        for i in 0..=x {
            let pos = (self.register_i + i as u16) as usize;
            let value = self.memory[pos];
            self.register_v[i as usize] = value;
        }
    }

    fn subroutine(&mut self, nnn: u16) {
        let msg = format!("subroutine called - nnn: {:#05x}", nnn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.push_stack(self.program_counter);
        self.program_counter = nnn;
    }

    fn sub_return(&mut self) {
        let msg = format!("sub return called");
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.program_counter = self.pop_stack();
    }

    fn jump(&mut self, nnn: u16) {
        let msg = format!("jump called - nnn: {:#05x}", nnn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.program_counter = nnn;
    }

    fn offset_jump(&mut self, nnn: u16) {
        let msg = format!("offset_jump called - nnn: {:#05x}", nnn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let offset = self.register_v[0x00];
        self.program_counter = nnn + offset as u16;
    }

    fn binary_or(&mut self, x: u8, y: u8) {
        let msg = format!("binary_or called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx | vy;
    }

    fn binary_and(&mut self, x: u8, y: u8) {
        let msg = format!("binary_and called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx & vy;
    }

    fn logical_xor(&mut self, x: u8, y: u8) {
        let msg = format!("logical_xor called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        self.register_v[x as usize] = vx ^ vy;
    }

    fn font_char(&mut self, x: u8) {
        let msg = format!("font_char called - x: {:#05x}", x,);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_i = (self.register_v[x as usize] as u16 * 5) + 0x50;
    }

    fn bcd_convert(&mut self, x: u8) {
        let vx = self.register_v[x as usize];
        self.memory[self.register_i as usize] = vx / 100;
        self.memory[self.register_i as usize + 1] = (vx / 10) % 10;
        self.memory[self.register_i as usize + 2] = vx % 10;
    }

    fn set_delay_timer(&mut self, x: u8) {
        let msg = format!("set_delay_timer called - x: {:#05x}", x,);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.delay_timer = self.register_v[x as usize];
    }

    fn set_sound_timer(&mut self, x: u8) {
        let msg = format!("set_sound_timer called - x: {:#05x}", x,);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.sound_timer = self.register_v[x as usize];
    }

    fn set_vx_to_delay(&mut self, x: u8) {
        let msg = format!("set_vx_to_delay called - x: {:#05x}", x,);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[x as usize] = self.delay_timer;
    }

    fn set_vx(&mut self, x: u8, nn: u8) {
        let msg = format!("set_vx called - x: {:#05x}, nn: {:#05x}", x, nn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[x as usize] = nn;
    }

    fn set_vx_vy(&mut self, x: u8, y: u8) {
        let msg = format!("set_vx_vy called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[x as usize] = self.register_v[y as usize];
    }
    
    fn add_vx(&mut self, x: u8, nn: u8) {
        let msg = format!("add_vx called - x: {:#05x}, nn: {:#05x}", x, nn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[x as usize] = self.register_v[x as usize].wrapping_add(nn);
    }

    fn add_vx_vy(&mut self, x: u8, y: u8) {
        let msg = format!("add_vx_vy called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize] as u16;
        let vy = self.register_v[y as usize] as u16; 
        let result = vx + vy;
        self.register_v[x as usize] = result as u8;

        self.register_v[0x0f] = if result > 255 {1} else {0};
    }

    fn vx_minus_vy(&mut self, x: u8, y: u8) {
        let msg = format!("vx_minus_vy called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        let result = vx - vy; 
        self.register_v[x as usize] = result;

        self.register_v[0x0f] = if vx > vy {1} else {0};
    }

    fn vy_minus_vx(&mut self, x: u8, y: u8) {
        let msg = format!("vy_minus_vx called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let vx = self.register_v[x as usize];
        let vy = self.register_v[y as usize];
        let result = vy - vx; 
        self.register_v[x as usize] = result;

        self.register_v[0x0f] = if vy > vx {1} else {0};
    }

    fn shift_left(&mut self, x: u8) {
        let msg = format!("shift_left called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[0x0f] = self.register_v[x as usize] & 1;
        self.register_v[x as usize] >>= 1;
    }

    fn shift_right(&mut self, x: u8) {
        let msg = format!("shift_right called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[0x0f] = (self.register_v[x as usize] & 0b10000000) >> 7;
        self.register_v[x as usize] <<= 1;
    }

    fn set_index(&mut self, nnn: u16) {
        let msg = format!("set_index called - x: {:#05x}", nnn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_i = nnn;
    }

    fn add_to_index(&mut self, x: u8) {
        let msg = format!("add_to_index called - x: {:#05x}", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_i += self.register_v[x as usize] as u16;
        if self.register_i > 0x0FFF {
            self.register_v[0x0f] = 1;
        }
    }

    fn skip_if_equal(&mut self, x: u8, nn: u8) {
        let msg = format!("skip_if_equal called - x: {:#05x}, nn: {:#05x}", x, nn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        if self.register_v[x as usize] == nn {
            self.program_counter += 2;
        }
    }

    fn skip_not_equal(&mut self, x: u8, nn: u8) {
        let msg = format!("skip_not_equal called - x: {:#05x}, nn: {:#05x}", x, nn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        if self.register_v[x as usize] != nn {
            self.program_counter += 2;
        }
    }

    fn skip_both_equal(&mut self, x: u8, y:u8) {
        let msg = format!("skip_both_equal called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        if self.register_v[x as usize] == self.register_v[y as usize] {
            self.program_counter += 2;
        }
    }

    fn skip_none_equal(&mut self, x: u8, y:u8) {
        let msg = format!("skip_none_equal called - x: {:#05x}, y: {:#05x}", x, y);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        if self.register_v[x as usize] != self.register_v[y as usize] {
            self.program_counter += 2;
        }
    }

    fn skip_if_key(&mut self, x: u8) {
        let msg = format!("skip_if_key called - x: {:#05x},", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let key = self.register_v[x as usize];
        if self.p_mem.key_list[key as usize] {
            self.program_counter += 2;
        } 
    }

    fn skip_not_key(&mut self, x: u8) {
        let msg = format!("skip_not_key called - x: {:#05x},", x);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let key = self.register_v[x as usize];
        if !self.p_mem.key_list[key as usize] {
            self.program_counter += 2;
        } 
    }

    fn random(&mut self, x: u8, nn: u8) {
        let msg = format!("random called - x: {:#05x}, nn: {:#05x}", x, nn);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        let num: u8 = rand::random();
        self.register_v[x as usize] = num & nn;
    }

    fn disp(&mut self, x: u8, y: u8, n: u8) {
        let msg = format!("display called - x: {:#05x}, y: {:#05x}, n: {:#05x}", x, y, n);
        self.p_mem.text.push(TextData::new(TextType::Function, msg));
        self.register_v[0x0f] = 0;
        for offset in 0..n {
            let y_coord = (self.register_v[y as usize] + offset) % Y_PIXELS as u8;

            for bit in 0..8 {
                let x_coord = (self.register_v[x as usize] + bit) % X_PIXELS as u8;
                let byte = (self.memory[(self.register_i + offset as u16) as usize] >> (7-bit)) & 1;

                if byte == 1 {
                    if self.p_mem.vram[y_coord as usize][x_coord as usize] == OFstate::ON {
                        self.p_mem.vram[y_coord as usize][x_coord as usize] = OFstate::OFF;
                        self.register_v[0x0F] = 1;
                    } else {
                        self.p_mem.vram[y_coord as usize][x_coord as usize] = OFstate::ON;
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

pub const MEM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
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
    memory: [u8; MEM_SIZE],
    stack: [u16; STACK_SIZE],
    sp: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            memory: [0; MEM_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    
    pub fn run(&mut self) { 
        self.load_font();
        self.dump_mem();
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
    }

    fn load_font(&mut self) {
        for i in 0..FONT.len() {
            self.memory[0x50 + i] = FONT[i];
        };
    }

    fn push_stack(&mut self, data: u16) {
        self.stack[self.sp] = data;
        self.sp += 1;
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.stack[self.sp];
        self.stack[self.sp] = 0;
        self.sp -= 1;
        data
    }
}




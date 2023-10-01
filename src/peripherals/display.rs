extern crate sdl2;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::utils::{X_PIXELS, Y_PIXELS};
use crate::utils::OFstate;

const TITLE: &str = "4ante's Chip-8 Emulator";
const BACKGROUND: Color = Color::BLACK;
const DRAW_COLOR: Color = Color::WHITE;

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = SCREEN_WIDTH / 2;
const PIXEL_SIZE: u32 = SCREEN_WIDTH / X_PIXELS;

pub struct Display {
    canvas: Canvas<Window>,
    event_pump: EventPump,
}

pub struct PeripheralMemory {
    pub vram: [[OFstate; 64];32],
    pub key_list: [bool; 16],
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window(TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
    
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Display { canvas, event_pump }
    }

    pub fn run(&mut self, p_mem: &mut PeripheralMemory) {
        self.handle_events(&mut p_mem.key_list);
        self.draw_vram(p_mem.vram);
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(BACKGROUND);
        self.canvas.clear();
    }

    fn handle_events(&mut self, key_list: &mut [bool]) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("Game Purposfully Quit :)")
                },

                Event::KeyDown { scancode, .. } => match scancode {
                    Some(Scancode::Num1) => key_list[0x01] = true,
                    Some(Scancode::Num2) => key_list[0x02] = true,
                    Some(Scancode::Num3) => key_list[0x03] = true,
                    Some(Scancode::Num4) => key_list[0x0c] = true,
                    Some(Scancode::Q) => key_list[0x04] = true,
                    Some(Scancode::W) => key_list[0x05] = true,
                    Some(Scancode::E) => key_list[0x06] = true,
                    Some(Scancode::R) => key_list[0x0d] = true,
                    Some(Scancode::A) => key_list[0x07] = true,
                    Some(Scancode::S) => key_list[0x08] = true,
                    Some(Scancode::D) => key_list[0x09] = true,
                    Some(Scancode::F) => key_list[0x0e] = true,
                    Some(Scancode::Z) => key_list[0x0a] = true,
                    Some(Scancode::X) => key_list[0x00] = true,
                    Some(Scancode::C) => key_list[0x0b] = true,
                    Some(Scancode::V) => key_list[0x0f] = true,
                    _ => break,
                },

                Event::KeyUp { scancode, .. } => match scancode {
                    Some(Scancode::Num1) => key_list[0x01] = false,
                    Some(Scancode::Num2) => key_list[0x02] = false,
                    Some(Scancode::Num3) => key_list[0x03] = false,
                    Some(Scancode::Num4) => key_list[0x0c] = false,
                    Some(Scancode::Q) => key_list[0x04] = false,
                    Some(Scancode::W) => key_list[0x05] = false,
                    Some(Scancode::E) => key_list[0x06] = false,
                    Some(Scancode::R) => key_list[0x0d] = false,
                    Some(Scancode::A) => key_list[0x07] = false,
                    Some(Scancode::S) => key_list[0x08] = false,
                    Some(Scancode::D) => key_list[0x09] = false,
                    Some(Scancode::F) => key_list[0x0e] = false,
                    Some(Scancode::Z) => key_list[0x0a] = false,
                    Some(Scancode::X) => key_list[0x00] = false,
                    Some(Scancode::C) => key_list[0x0b] = false,
                    Some(Scancode::V) => key_list[0x0f] = false,
                    _ => break
                },
        
                _ => {}
            }
        }
        true
    }

    pub fn draw_vram(&mut self, vram: [[OFstate; 64]; 32]) {
        self.clear();
        for y in 0..Y_PIXELS {
            let screen_y = y as u32 * PIXEL_SIZE;
            for x in 0..X_PIXELS {
                let screen_x = x as u32 * PIXEL_SIZE;
                match vram[y as usize][x as usize] {
                    OFstate::ON => self.canvas.set_draw_color(DRAW_COLOR),
                    OFstate::OFF => self.canvas.set_draw_color(BACKGROUND),
                }

                self.canvas.fill_rect(Rect::new(screen_x as i32, screen_y as i32, PIXEL_SIZE, PIXEL_SIZE)).unwrap();
            }
        }
    }
}

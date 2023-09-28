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

    pub fn run(&mut self, vram: [[OFstate; 64]; 32]) {
        self.handle_events();
        self.draw_vram(vram);

        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(BACKGROUND);
        self.canvas.clear();
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("Game Purposfully Quit :)")
                },

                Event::KeyDown { scancode: Some(Scancode::Num1), .. } => {
                    println!("1");
                },
                Event::KeyDown { scancode: Some(Scancode::Num2), .. } => {
                    println!("2");
                },
                Event::KeyDown { scancode: Some(Scancode::Num3), .. } => {
                    println!("3");
                },
                Event::KeyDown { scancode: Some(Scancode::Num4), .. } => {
                    println!("C");
                },
                Event::KeyDown { scancode: Some(Scancode::Q), .. } => {
                    println!("4");
                },
                Event::KeyDown { scancode: Some(Scancode::W), .. } => {
                    println!("5");
                },
                Event::KeyDown { scancode: Some(Scancode::E), .. } => {
                    println!("6");
                },
                Event::KeyDown { scancode: Some(Scancode::R), .. } => {
                    println!("D");
                },
                Event::KeyDown { scancode: Some(Scancode::A), .. } => {
                    println!("7");
                },
                Event::KeyDown { scancode: Some(Scancode::S), .. } => {
                    println!("8");
                },
                Event::KeyDown { scancode: Some(Scancode::D), .. } => {
                    println!("9");
                },
                Event::KeyDown { scancode: Some(Scancode::F), .. } => {
                    println!("E");
                },
                Event::KeyDown { scancode: Some(Scancode::Z), .. } => {
                    println!("A");
                },
                Event::KeyDown { scancode: Some(Scancode::X), .. } => {
                    println!("0");
                },
                Event::KeyDown { scancode: Some(Scancode::C), .. } => {
                    println!("B");
                },
                Event::KeyDown { scancode: Some(Scancode::V), .. } => {
                    println!("F");
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
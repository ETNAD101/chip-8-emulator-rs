extern crate sdl2;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use std::time::Duration;

use crate::utils::OFstate;

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = SCREEN_WIDTH / 2;
const X_PIXELS: u32 = 64;
const Y_PIXELS: u32 = 32;
const PIXEL_SIZE: u32 = SCREEN_WIDTH / X_PIXELS;
const FPS: u32 = 60;


pub struct Display {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    pub pixel_state: [[Pixel; 64]; 32],
}

#[derive(Clone, Copy)]
pub struct Pixel {
    pub state: OFstate,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("rust-sdl2 demo", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
    
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Display { canvas, event_pump, pixel_state: [[Pixel{state: OFstate::OFF}; 64]; 32]}
    }

    pub fn run(&mut self) {
        //remember to remove dupe
        //while self.handle_events {
        self.handle_events();
        self.clear();

        self.canvas.present();
           // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
       //}
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
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

    pub fn toggle_pixel(&mut self, x: u8, y: u8, state: OFstate) {
        let screen_x = x as u32 * PIXEL_SIZE;
        let screen_y = y as u32 * PIXEL_SIZE;

        match state {
            OFstate::ON => self.canvas.set_draw_color(Color::WHITE),
            OFstate::OFF => self.canvas.set_draw_color(Color::BLACK),
        }
        
        self.canvas.fill_rect(Rect::new(screen_x as i32, screen_y as i32, PIXEL_SIZE, PIXEL_SIZE));
        self.pixel_state[x as usize][y as usize].state = state;
    }
    
}


extern crate sdl2;

use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const SCREEN_WIDTH: u32 = 960;
const SCREEN_HEIGHT: u32 = SCREEN_WIDTH / 2;
const X_PIXELS: u32 = 64;
const Y_PIXELS: u32 = 32;
const PIXEL_SIZE: u32 = SCREEN_WIDTH / X_PIXELS;
const FPS: u32 = 6;

const ON: u8 = 1;
const OFF: u8 = 0;



pub struct Display {
    canvas: Canvas<Window>,
    event_pump: EventPump,
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

        Display { canvas, event_pump }
    }

    pub fn run(&mut self) {
        while self.handle_events() {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            
            for x in 0..32 {
                for y in 0..16 {
                    self.toggle_pixel(x as u32, y as u32, ON);
                }
            }
            
            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
        }
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                },
                _ => {}
            }
        }
        true
    }

    fn toggle_pixel(&mut self, x: u32, y: u32, state: u8) {
        let screen_x = x * PIXEL_SIZE;
        let screen_y = y * PIXEL_SIZE;
        
        if state == ON {
            self.canvas.set_draw_color(Color::WHITE);
        } else {
            self.canvas.set_draw_color(Color::BLACK);
        }
        
        self.canvas.fill_rect(Rect::new(screen_x as i32, screen_y as i32, PIXEL_SIZE, PIXEL_SIZE));
    }
    
}


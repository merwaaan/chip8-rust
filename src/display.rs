extern crate minifb;

use minifb::{Scale, Window, WindowOptions};

pub struct Display
{
    window: Window,
    buffer: Vec<u32>
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0x00000000;

impl Display
{
    pub fn new() -> Self
    {
        let window_options = WindowOptions
        {
            scale: Scale::X8,
            ..WindowOptions::default()
        };

        let window = Window::new("chip8-rust", WIDTH, HEIGHT, window_options).unwrap_or_else(|e|
        {
            panic!("{}", e);
        });

        let mut buffer: Vec<u32> = vec![BLACK; WIDTH * HEIGHT];

        Self
        {
            window,
            buffer
        }
    }

    pub fn is_running(&self) -> bool
    {
        self.window.is_open()
    }

    // Returns 1 if this erases an existing pixel
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> u8
    {
        let mut erased = false;

        for (line_index, line) in sprite.iter().enumerate()
        {
            for pixel in 0..8
            {
                let pixel_index = (y + line_index as u8) as usize * WIDTH + (x + pixel) as usize;

                let state_buffer = self.buffer[pixel_index] == WHITE;
                let state_sprite = (line & (1 << (7 - pixel))) > 0;

                let state_next = state_buffer ^ state_sprite;
                self.buffer[pixel_index] = if state_next { WHITE } else { BLACK };

                erased |= state_buffer && !state_sprite;
            }
        }

        erased as u8
    }

    pub fn clear(&mut self)
    {
        self.buffer.iter_mut().map(|x| *x = BLACK);
    }

    pub fn update(&mut self)
    {
        self.window.update_with_buffer(&self.buffer).unwrap();
    }
}

extern crate minifb;

use minifb::{Scale, Window, WindowOptions};

pub struct Display
{
    window: Window,
    buffer: Vec<u32>
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

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

        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

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

    pub fn set_pixel(&mut self, x: usize, y: usize, state: bool)
    {
        self.buffer[y * WIDTH + x] = if state { 0xFFFFFFFF } else { 0x0 };
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool
    {
        return self.buffer[y * WIDTH + x] > 0;
    }

    pub fn update(&mut self)
    {
        self.window.update_with_buffer(&self.buffer).unwrap();
    }
}

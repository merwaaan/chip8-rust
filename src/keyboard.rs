//extern crate minifb;

//use minifb::{Key, Window};

pub struct Keyboard
{
    keys: [bool; 9]
}

impl Keyboard
{
    pub fn new(/*window: &Window*/) -> Keyboard
    {
        //window.set_input_callback();
        Keyboard
        {
             keys: [false; 9]
        }
    }

    pub fn press(&mut self, key: usize)
    {
        self.keys[key] = true;
    }

    pub fn release(&mut self, key: usize)
    {
        self.keys[key] = false;
    }

    pub fn is_pressed(&self, key: usize) -> bool
    {
        self.keys[key]
    }
}

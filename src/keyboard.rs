//extern crate minifb;

//use minifb::{Key, Window};

pub struct Keyboard
{
    keys: [bool; 9],
    //wait_callback: Option<Fn(u8)>
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

    pub fn press(&mut self, key: u8)
    {
        self.keys[key as usize] = true;
    }

    pub fn release(&mut self, key: u8)
    {
        self.keys[key as usize] = false;
    }

    pub fn is_pressed(&self, key: u8) -> bool
    {
        self.keys[key as usize]
    }

    /*pub fn wait<F>(&mut self, callback: F)
        where F: Fn(u8)
    {

    }*/

    pub fn is_waiting(&self) -> bool
    {
        false
    }
}

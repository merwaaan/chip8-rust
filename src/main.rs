mod audio;
mod core;
mod display;
mod keyboard;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main()
{
    // Load the program

    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        panic!();
    }

    let mut file = File::open(&args[1])
        .expect("Cannot open program");
    let mut program = Vec::new();
    file.read_to_end(&mut program)
        .expect("Cannot read program");

    // Setup the Chip-8

    let mut chip8 = core::Chip8::new();
    chip8.load_program(&program);

    while chip8.is_running()
    {
        chip8.update();
    }
}
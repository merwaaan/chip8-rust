mod audio;
mod core;
mod display;
mod keyboard;

fn main()
{
    let mut chip8 = core::Chip8::new();

    while chip8.is_running()
    {
        chip8.update();
    }
}
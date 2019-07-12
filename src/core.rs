extern crate rand;

use crate::audio::Audio;
use crate::display::Display;
use crate::keyboard::Keyboard;

use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};

pub struct Chip8
{
    // Memory
    mem: [u8; 0x1000],

    // Registers
    v: [u8; 16],
    i: u16,

    // Program counter
    pc: u16,

    // Stack & stack pointer
    stack: [u16; 16],
    sp: u16,

    // Timers
    dt: u8,
    st: u8,

    // Randomness
    rng: ThreadRng,
    rng_distrib: Uniform<u8>,

    display: Display,
    keyboard: Keyboard,
    audio: Audio,

    running: bool
}

impl Chip8
{
    pub fn new() -> Chip8
    {
        Chip8
        {
            mem: [0; 0x1000],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            rng: rand::thread_rng(),
            rng_distrib: Uniform::new_inclusive(0, 0xFF),
            keyboard: Keyboard::new(),
            display: Display::new(),
            audio: Audio::new(),
            running: true
        }
    }

    pub fn is_running(&self) -> bool
    {
        self.running && self.display.is_running()
    }

    pub fn load_program(&mut self, program: &[u8])
    {
        // Load the program
        let mut program_mem = &mut self.mem[0x200 .. 0x200 + program.len()];
        program_mem.copy_from_slice(program);

        // Load the font
        let mut font_mem = &mut self.mem[0 .. FONT.len()];
        font_mem.copy_from_slice(&FONT);
    }

    pub fn update(&mut self)
    {
        self.step();
        self.display.update();
    }

    pub fn step(&mut self)
    {
        let opcode = [
            (self.mem[self.pc as usize] & 0xF0) >> 4,
            self.mem[self.pc as usize] & 0x0F,
            (self.mem[self.pc as usize + 1] & 0xF0) >> 4,
            self.mem[self.pc as usize + 1] & 0x0F
        ];

        println!("{:X}:   {:x}{:x}{:x}{:x}", self.pc, opcode[0], opcode[1], opcode[2], opcode[3]);

        match opcode
        {
            [0, 0, 0xE, 0] => // CLS
            {
                println!("CLS");
                // TODO
            },
            [0, 0, 0xE, 0xE] => // RET
            {
                println!("RET");
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },
            [1, n1, n2, n3] => // JP addr
            {
                println!("JP");
                self.pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16); // TODO -2?
            },
            [2, n1, n2, n3] => // CALL addr
            {
                println!("CALL");
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16); // TODO -2?
            },
            [3, x, k1, k2] => // SE Vx, byte
            {
                println!("SE");
                if self.v[x as usize] == (k1 << 4) | k2
                {
                    self.pc += 2;
                }
            },
            [4, x, k1, k2] => // SNE Vx, byte
            {
                println!("SNE");
                if self.v[x as usize] != (k1 << 4) | k2
                {
                    self.pc += 2;
                }
            },
            [5, x, y, 0] => // SE Vx, Vy
            {
                println!("SE");
                if self.v[x as usize] == self.v[y as usize]
                {
                    self.pc += 2;
                }
            },
            [6, x, k1, k2] => // LD Vx, byte
            {
                println!("LD");
                self.v[x as usize] = (k1 << 4) | k2;
            },
            [7, x, k1, k2] => // ADD Vx, byte
            {
                println!("ADD");
                self.v[x as usize] += (k1 << 4) | k2;
            },
            [8, x, y, 0] => // LD Vx, Vy
            {
                println!("LD");
                self.v[x as usize] = self.v[y as usize];
            },
            [8, x, y, 1] => // OR Vx, Vy
            {
                println!("OR");
                self.v[x as usize] |= self.v[y as usize];
            },
            [8, x, y, 2] => // AND Vx, Vy
            {
                println!("AND");
                self.v[x as usize] &= self.v[y as usize];
            },
            [8, x, y, 3] => // XOR Vx, Vy
            {
                println!("XOR");
                self.v[x as usize] ^= self.v[y as usize];
            },
            [8, x, y, 4] => // ADD Vx, Vy
            {
                println!("ADD");
                let r = self.v[x as usize] + self.v[y as usize];
                self.v[0xF] = if r < self.v[x as usize] { 1 } else { 0 };
                self.v[x as usize] = r;
            },
            [8, x, y, 5] => // SUB Vx, Vy
            {
                println!("SUB");
                self.v[0xF] = if self.v[x as usize] > self.v[y as usize] { 1 } else { 0 };
                self.v[x as usize] = self.v[x as usize] - self.v[y as usize];
            },
            [8, x, y, 6] => // SHR Vx , Vy
            {
                println!("SHR");
                self.v[0xF] = self.v[x as usize] & 0x1;
                self.v[x as usize] >>= 1;
            },
            [8, x, y, 7] => // SUBN Vx, Vy
            {
                println!("SUBN");
                self.v[0xF] = if self.v[y as usize] > self.v[x as usize] { 1 } else { 0 };
                self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
            },
            [8, x, y, 0xE] => // SHL Vx, Vy
            {
                println!("SHL");
                self.v[0xF] = self.v[x as usize] & 0x8;
                self.v[x as usize] <<= 1;
            },
            [9, x, y, 0] => // SNE Vx, Vy
            {
                println!("SNE");
                if self.v[x as usize] != self.v[y as usize]
                {
                    self.pc += 2;
                }
            },
            [0xA, n1, n2, n3] => // LD I, addr
            {
                println!("LD");
                self.i = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
            },
            [0xB, n1, n2, n3] => // JP V0, addr
            {
                println!("JP");
                self.pc = (self.v[0] as u16) + ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
            },
            [0xC, x, k1, k2] => // RND Vx, byte
            {
                println!("RND");
                self.v[x as usize] = self.rng_distrib.sample(&mut self.rng) & ((k1 << 4) | k2)
            },
            [0xD, x, y, n] =>
            {
                println!("DRW");
                // TODO
            },
            [0xE, x, 0x9, 0xE] => // SKP Vx
            {
                println!("SKP");
                if self.v[x as usize] == 0 // TODO
                {
                    self.pc += 2;
                }
            },

            [0xE, x, 0xA, 0x1] => // SKNP Vx
            {
                println!("SKNP");
                if self.v[x as usize] != 0 // TODO
                {
                    self.pc += 2;
                }
            },
            [0xF, x, 0x0, 0x7] => // LD Vx, DT
            {
                println!("LD");
                self.v[x as usize] = self.dt;
            },
            [0xF, x, 0x0, 0xA] => // LD Vx, K
            {
                println!("LD");
                self.v[x as usize] = 0;// TODO
            },
            [0xF, x, 0x1, 0x5] => // LD DT, Vx
            {
                println!("LD");
                self.dt = self.v[x as usize];
            },
            [0xF, x, 0x0, 0x8] => // LD ST, Vx
            {
                println!("LD");
                self.st = self.v[x as usize];
            },
            [0xF, x, 0x1, 0xE] => // ADD I, Vx
            {
                println!("ADD");
                self.i += self.v[x as usize] as u16;
            },
            [0xF, x, 0x2, 0x9] => // LD F, Vx
            {
                println!("LD");
                // TODO
            },
            [0xF, x, 0x3, 0x3] => // LD B, Vx
            {
                println!("LD");
                self.mem[self.i as usize] = self.v[x as usize] / 100;
                self.mem[(self.i + 1) as usize] = self.v[x as usize] / 10 % 10;
                self.mem[(self.i + 2) as usize] = self.v[x as usize] % 10;
            },
            [0xF, x, 0x5, 0x5] => // LD [I], Vx
            {
                println!("LD");
                for n in 0..=x
                {
                    self.mem[(self.i + n as u16) as usize] = self.v[n as usize];
                }

            },
            [0xF, x, 0x6, 0x5] => // LD Vx, [I]
            {
                println!("LD");
                for n in 0..=x
                {
                    self.v[n as usize] = self.mem[(self.i + n as u16) as usize];
                }
            },
            _ => panic!("unknown opcode {:x}{:x}{:x}{:x}", opcode[0], opcode[1], opcode[2], opcode[3])
        }

        // Increment the program counter
        self.pc += 2;

        // Decrement the timers
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; }
    }
}

const FONT: [u8; 5*16] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];

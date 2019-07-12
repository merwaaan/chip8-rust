fn main()
{
    // Memory
    let mut mem: [u8; 0x1000] = [0; 0x1000];

    // Registers
    let mut v: [u8; 16] = [0; 16];
    let mut i: u16 = 0;

    // Program counter
    let mut pc: u16 = 0;

    // Stack & stack pointer
    let mut stack: [u16; 16] = [0; 16];
    let mut sp: u16 = 0;

    // Timers
    let mut dt: u8 = 0;
    let mut st: u8 = 0;

    loop
    {
        let opcode = [
            (mem[pc as usize] & 0xF0) >> 4,
            mem[pc as usize] & 0x0F,
            (mem[pc as usize + 1] & 0xF0) >> 4,
            mem[pc as usize + 1] & 0x0F
        ];

        println!("{}", opcode[0]);

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
                pc = stack[sp as usize];
                sp -= 1;
            },
            [1, n1, n2, n3] => // JP addr
            {
                println!("JP");
                pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16); // TODO -2?
            },
            [2, n1, n2, n3] => // CALL addr
            {
                println!("CALL");
                sp += 1;
                stack[sp as usize] = pc;
                pc = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16); // TODO -2?
            },
            [3, x, k1, k2] => // SE Vx, byte
            {
                println!("SE");
                if v[x as usize] == (k1 << 4) | k2
                {
                    pc += 2;
                }
            },
            [4, x, k1, k2] => // SNE Vx, byte
            {
                println!("SNE");
                if v[x as usize] != (k1 << 4) | k2
                {
                    pc += 2;
                }
            },
            [5, x, y, 0] => // SE Vx, Vy
            {
                println!("SE");
                if v[x as usize] == v[y as usize]
                {
                    pc += 2;
                }
            },
            [6, x, k1, k2] => // LD Vx, byte
            {
                println!("LD");
                v[x as usize] = (k1 << 4) | k2;
            },
            [7, x, k1, k2] => // ADD Vx, byte
            {
                println!("ADD");
                v[x as usize] += (k1 << 4) | k2;
            },
            [8, x, y, 0] => // LD Vx, Vy
            {
                println!("LD");
                v[x as usize] = v[y as usize];
            },
            [8, x, y, 1] => // OR Vx, Vy
            {
                println!("OR");
                v[x as usize] |= v[y as usize];
            },
            [8, x, y, 2] => // AND Vx, Vy
            {
                println!("AND");
                v[x as usize] &= v[y as usize];
            },
            [8, x, y, 3] => // XOR Vx, Vy
            {
                println!("XOR");
                v[x as usize] ^= v[y as usize];
            },
            [8, x, y, 4] => // ADD Vx, Vy
            {
                println!("ADD");
                let r = v[x as usize] + v[y as usize];
                v[0xF] = if r < v[x as usize] { 1 } else { 0 };
                v[x as usize] = r;
            },
            [8, x, y, 5] => // SUB Vx, Vy
            {
                println!("SUB");
                v[0xF] = if v[x as usize] > v[y as usize] { 1 } else { 0 };
                v[x as usize] = v[x as usize] - v[y as usize];
            },
            [8, x, y, 6] => // SHR Vx , Vy
            {
                println!("SHR");
                v[0xF] = v[x as usize] & 0x1;
                v[x as usize] >>= 1;
            },
            [8, x, y, 7] => // SUBN Vx, Vy
            {
                println!("SUBN");
                v[0xF] = if v[y as usize] > v[x as usize] { 1 } else { 0 };
                v[x as usize] = v[y as usize] - v[x as usize];
            },
            [8, x, y, 0xE] => // SHL Vx, Vy
            {
                println!("SHL");
                v[0xF] = v[x as usize] & 0x8;
                v[x as usize] <<= 1;
            },
            [9, x, y, 0] => // SNE Vx, Vy
            {
                println!("SNE");
                if v[x as usize] != v[y as usize]
                {
                    pc += 2;
                }
            },
            [0xA, n1, n2, n3] => // LD I, addr
            {
                println!("LD");
                i = ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
            },
            [0xB, n1, n2, n3] => // JP V0, addr
            {
                println!("JP");
                pc = (v[0] as u16) + ((n1 as u16) << 8) | ((n2 as u16) << 4) | (n3 as u16);
            },
            [0xC, x, k1, k2] => // RND Vx, byte
            {
                println!("RND");
                // TODO
            },
            [0xD, x, y, n] =>
            {
                println!("DRW");
                // TODO
            },
            [0xE, x, 0x9, 0xE] => // SKP Vx
            {
                println!("SKP");
                if v[x as usize] == 0 // TODO
                {
                    pc += 2;
                }
            },

            [0xE, x, 0xA, 0x1] => // SKNP Vx
            {
                println!("SKNP");
                if v[x as usize] != 0 // TODO
                {
                    pc += 2;
                }
            },
            [0xF, x, 0x0, 0x7] => // LD Vx, DT
            {
                println!("LD");
                v[x as usize] = dt;
            },
            [0xF, x, 0x0, 0xA] => // LD Vx, K
            {
                println!("LD");
                v[x as usize] = 0;// TODO
            },
            [0xF, x, 0x1, 0x5] => // LD DT, Vx
            {
                println!("LD");
                dt = v[x as usize];
            },
            [0xF, x, 0x0, 0x8] => // LD ST, Vx
            {
                println!("LD");
                st = v[x as usize];
            },
            [0xF, x, 0x1, 0xE] => // ADD I, Vx
            {
                println!("ADD");
                i += v[x as usize] as u16;
            },
            [0xF, x, 0x2, 0x9] => // LD F, Vx
            {
                println!("LD");
                // TODO
            },
            [0xF, x, 0x3, 0x3] => // LD B, Vx
            {
                println!("LD");
                mem[i as usize] = v[x as usize] / 100;
                mem[(i + 1) as usize] = v[x as usize] / 10 % 10;
                mem[(i + 2) as usize] = v[x as usize] % 10;
            },
            [0xF, x, 0x5, 0x5] => // LD [I], Vx
            {
                println!("LD");
                for n in 0..=x
                {
                    mem[(i + n as u16) as usize] = v[n as usize];
                }

            },
            [0xF, x, 0x6, 0x5] => // LD Vx, [I]
            {
                println!("LD");
                for n in 0..=x
                {
                    v[n as usize] = mem[(i + n as u16) as usize];
                }
            },
            _ => panic!("unknown opcode")
        }

        // Increment the program counter
        pc += 2;

        // Decrement the timers
        if dt > 0 { dt -= 1; }
        if st > 0 { st -= 1; }
    }
}

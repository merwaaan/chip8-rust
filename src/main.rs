fn main()
{
    let mut mem: [u8; 0x1000] = [0; 0x1000];
    let mut v: [u8; 16] = [0; 16];
    let mut i: u16 = 0;
    let mut pc: usize = 0;


    loop
    {
        let opcode = [
            (mem[pc] & 0xF0) >> 4,
            mem[pc] & 0x0F,
            (mem[pc+1] & 0xF0) >> 4,
            mem[pc+1] & 0x0F
        ];

        println!("{}", opcode[0]);

        match opcode
        {
            [0, n1, n2, n3] => println!("SYS"),
            [0, 0, 0xE, 0] => println!("CLS"),
            [0, 0, 0xE, 0xE] => println!("RET"),
            [1, n1, n2, n3] => println!("JMP"),
            [2, n1, n2, n3] => println!("CALL"),
            _ => panic!("unknown opcode")
        }

        pc += 2;
    }
}

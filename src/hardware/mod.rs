use std::ops::{Index, IndexMut};

const START_CHIP_8_PROGRAM: usize = 0x200;

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    registers: [u8; 16],
    sp: u8,
    pc: u16,
    i: u16,
    dt: u8,
    st: u8,
}

impl Index<u16> for Chip8 {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.memory[index as usize]
    }
}

impl Index<u8> for Chip8 {
    type Output = u8;

    fn index(&self, index: u8) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<u8> for Chip8 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Chip8 {
    pub fn new(data: &[u8]) -> Self {
        let mut memory = [0; 4096];

        memory[..FONT.len()].copy_from_slice(&FONT);
        memory[START_CHIP_8_PROGRAM..START_CHIP_8_PROGRAM + data.len()].copy_from_slice(data);

        Self {
            memory,
            registers: [0; 16],
            stack: [0; 16],
            sp: 0,
            pc: START_CHIP_8_PROGRAM as u16,
            i: 0,
            dt: 0,
            st: 0,
        }
    }

    pub fn fetch(&mut self) -> (u8, u8) {
        let byte = self[self.pc];
        self.pc += 1;

        let a = byte >> 4;
        let b = byte & 0x0F;

        (a, b)
    }

    pub fn ret(&mut self) {
        let sp = self.sp - 1;
        self.pc = self.stack[sp as usize];
        self.sp = sp
    }

    pub fn jmp(&mut self, nnn: u16) {
        self.pc = nnn
    }

    pub fn call(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn
    }

    pub fn eq(&mut self, x: u8, nn: u8) {
        let x = self[x];
        if x == nn {
            self.pc += 1;
        }
    }

    pub fn neq(&mut self, x: u8, nn: u8) {
        let x = self[x];
        if x != nn {
            self.pc += 1;
        }
    }

    pub fn skip(&mut self, x: u8, y: u8) {
        let y = self[y];
        self.eq(x, y)
    }

    pub fn setx(&mut self, x: u8, nn: u8) {
        self[x] = nn
    }

    pub fn addn(&mut self, x: u8, nn: u8) {
        let v = self[x];
        self[x] = v + nn
    }

    pub fn stxy(&mut self, x: u8, y: u8) {
        self[x] = self[y]
    }

    pub fn or(&mut self, x: u8, y: u8) {
        let vx = self[x];
        let vy = self[y];
        self[x] = vx | vy
    }

    pub fn and(&mut self, x: u8, y: u8) {
        let vx = self[x];
        let vy = self[y];
        self[x] = vx & vy
    }

    pub fn xor(&mut self, x: u8, y: u8) {
        let vx = self[x];
        let vy = self[y];
        self[x] = vx ^ vy
    }

    pub fn addx(&mut self, x: u8, y: u8) {
        let vx = self[x];
        let vy = self[y];
        let v = vx.wrapping_add(vy);
        self[0xFu8] = if v < vx { 1 } else { 0 };
        self[x] = v
    }

    pub fn subx(&mut self, x: u8, y: u8) {
        let vx = self[x];
        let vy = self[y];
        let v = vx.wrapping_sub(vy);
        self[0xFu8] = if v > vx { 1 } else { 0 };
        self[x] = v
    }

    pub fn shir(&mut self, x: u8) {
        let v = self[x];
        self[0xFu8] = v & 0b00000001;
        self[x] = v >> 1
    }

    pub fn shil(&mut self, x: u8) {
        let v = self[x];
        self[0xFu8] = v >> (u8::BITS - 1);
        self[x] = v << 1
    }

    pub fn seti(&mut self, nnn: u16) {
        self.i = nnn
    }

    pub fn jmpi(&mut self, nnn: u16) {
        let v0 = self[0u8] as u16;
        self.pc = v0 + nnn
    }
}

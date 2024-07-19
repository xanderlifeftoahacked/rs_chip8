use crate::constants::*;
use crate::utils::*;
use rand::prelude::*;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    video: [u32; 64 * 32],
    opcode: u16,
    rand_gen: ThreadRng,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; 64 * 32],
            opcode: 0,
            rand_gen: thread_rng(),
        }
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut ret = Chip8 {
            ..Default::default()
        };
        for i in 0..FONTSET_SIZE as usize {
            ret.memory[i + FONTSET_START_ADDRESS as usize] = FONTSET[i];
        }
        ret
    }

    pub fn get_random_byte(&mut self) -> u8 {
        self.rand_gen.gen_range(0..=255)
    }

    pub fn load_rom(&mut self, filename: &str) -> io::Result<()> {
        let mut file = File::open(filename)?;

        let file_size = file.seek(SeekFrom::End(0))?;
        let mut buffer = vec![0; file_size as usize];

        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut buffer)?;
        drop(file);

        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[START_ADDRESS as usize + i] = byte;
        }

        Ok(())
    }

    pub fn op_00e0(&mut self) {
        self.video.fill_with(|| 0);
    }
    pub fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }
    pub fn op_1nnn(&mut self) {
        let addr = get_low_12_bits(self.opcode);
        self.pc = addr;
    }
    pub fn op_2nnn(&mut self) {
        let addr = get_low_12_bits(self.opcode);
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = addr;
    }
    pub fn op_3xkk(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let byte = get_low_byte(self.opcode);

        if self.registers[vx] == byte {
            self.pc += 2;
        }
    }
    pub fn op_4xkk(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let byte = get_low_byte(self.opcode);

        if self.registers[vx] == byte {
            self.pc += 2;
        }
    }
    pub fn op_5xy0(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;

        if self.registers[vx] == self.registers[vy] {
            self.pc += 2;
        }
    }
    pub fn op_6xkk(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let byte = get_low_byte(self.opcode);
        self.registers[vx] = byte;
    }
    pub fn op_7xkk(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let byte = get_low_byte(self.opcode);
        self.registers[vx] += byte;
    }
    pub fn op_8xy0(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[vx] = self.registers[vy];
    }
    pub fn op_8xy1(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[vx] |= self.registers[vy]
    }
    pub fn op_8xy2(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[vx] &= self.registers[vy]
    }
    pub fn op_8xy3(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[vx] ^= self.registers[vy]
    }
    pub fn op_8xy4(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        let res = self.registers[vx] as u16 + self.registers[vy] as u16;
        self.registers[0xF] = if res > 0xFF { 1 } else { 0 };
        self.registers[vx] = get_low_byte(res);
    }
    pub fn op_8xy5(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[0xF] = if self.registers[vx] > self.registers[vy] {
            1
        } else {
            0
        };
        self.registers[vx] -= self.registers[vy];
    }
    pub fn op_8xy6(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        self.registers[0xF] = self.registers[vx] & 0x1;
        self.registers[vx] >>= 1;
    }
    pub fn op_8xy7(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;
        self.registers[0xF] = if self.registers[vy] > self.registers[vx] {
            1
        } else {
            0
        };
        self.registers[vx] = self.registers[vy] - self.registers[vx];
    }
    pub fn op_8xye(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        self.registers[0xF] = (self.registers[vx] & 0x80) >> 7;
        self.registers[vx] <<= 1;
    }
    pub fn op_9xy0(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;

        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }
    pub fn op_annn(&mut self) {
        self.index = get_low_12_bits(self.opcode);
    }
    pub fn op_bnnn(&mut self) {
        self.pc = self.registers[0] as u16 + get_low_12_bits(self.opcode);
    }
    pub fn op_cxkk(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let byte = get_low_byte(self.opcode);
        self.registers[vx] = self.get_random_byte() & byte;
    }
    pub fn op_dxyn(&mut self) {
        let vx = get_low_4_bits_of_high_byte(self.opcode) as usize;
        let vy = get_high_4_bits_of_low_byte(self.opcode) as usize;

        let sprite_height = get_lowest_4_bits(self.opcode);
        let x_coord = (self.registers[vx] % VIDEO_WIDTH) as usize;
        let y_coord = (self.registers[vy] % VIDEO_HEIGHT) as usize;

        self.registers[0xF] = 0;

        for row in 0..sprite_height as usize{
            let sprite_byte = self.memory[self.index as usize + row];
            
            for col in 0..8 as usize{
                let sprite_pixel = sprite_byte & (0x80 >> col);
                let screen_pixel = self.video[((y_coord + row) * VIDEO_WIDTH as usize + x_coord + col) as usize]
            }
        }

    }
    pub fn op_ex9e(&mut self) {}
    pub fn op_exa1(&mut self) {}
    pub fn op_fx07(&mut self) {}
    pub fn op_fx0a(&mut self) {}
    pub fn op_fx15(&mut self) {}
    pub fn op_fx18(&mut self) {}
    pub fn op_fx1e(&mut self) {}
    pub fn op_fx29(&mut self) {}
    pub fn op_fx33(&mut self) {}
    pub fn op_fx55(&mut self) {}
    pub fn op_fx65(&mut self) {}
}

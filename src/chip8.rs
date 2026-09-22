use std::error::Error;
use std::fs;

use crate::instruction::Instruction;
use crate::io::*;

const START_IND: usize = 0x200;

pub struct Chip8 {
    memory: [u8; 0xFFF + 1],
    v: [u8; 0xF + 1],
    i: u16,    // stack pointer
    pc: usize, // original u16
    sp: u8,
    stack: [u16; 16],
    grid: [[bool; 64]; 32],
    dt: u8,
    io: Box<dyn Io>,
}

impl Chip8 {
    pub fn new(rom_data: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut memory = [0; 0xFFF + 1];

        if START_IND + rom_data.len() > memory.len() {
            return Err("Err file to big".into());
        }

        memory[START_IND..START_IND + rom_data.len()].copy_from_slice(rom_data);

        Ok(Self {
            memory,
            v: [0; 0xF + 1],
            i: 0,
            pc: START_IND,
            sp: 0,
            stack: [0; 16],
            grid: [[false; 64]; 32],
            dt: 0,
            io: Box::new(RaylibIo::new()),
        })
    }

    pub fn print_rom_hex(&mut self) {
        while let Some(instruction) = self.fetch() {
            println!("{:04X}", instruction);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(instruction) = self.fetch() {
            if let Some(result) = self.io.check_update()
                && result == Change::Exit
            {
                break;
            }

            self.instruct(instruction)?;

            self.io.render(self.grid);
        }
        Ok(())
    }

    fn fetch(&mut self) -> Option<Instruction> {
        if self.pc + 1 >= self.memory.len() {
            return None;
        }

        let byte_one = self.memory[self.pc];
        self.pc += 1;
        let byte_two = self.memory[self.pc];
        self.pc += 1;

        Some(Instruction::new(byte_one, byte_two))
    }

    fn instruct(&mut self, instruction: Instruction) -> Result<(), Box<dyn Error>> {
        match instruction {
            // cls
            Instruction(0x00E0) => {
                self.grid = [[false; 64]; 32];
                Ok(())
            }
            // RET
            Instruction(0x00EE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize] as usize;
                Ok(())
            }
            // SYS addr
            Instruction(_) if instruction.get_first() == 0x0 => Ok(()),
            // LD I, adr
            Instruction(_) if instruction.get_first() == 0xA => {
                self.i = instruction.get_nnn();
                Ok(())
            }
            // JP V0, addr
            Instruction(_) if instruction.get_first() == 0xB => {
                let nnn = instruction.get_nnn() as usize;
                self.pc = nnn + (self.v[0] as usize);
                Ok(())
            }
            //DRW Vx, Vy, nibble
            Instruction(_) if instruction.get_first() == 0xD => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;
                let n = instruction.get_n() as usize;
                let i = self.i as usize;

                let start_x = self.v[x] as usize;
                let start_y = self.v[y] as usize;

                self.v[0xF] = 0;

                for (row, byte) in self.memory[i..i + n].iter().enumerate() {
                    for bit in (0..8).rev() {
                        if (byte >> bit) & 1 == 1 {
                            let px = (start_x + (7 - bit)) % 64;
                            let py = (start_y + row) % 32;

                            if self.grid[py][px] {
                                self.v[0xF] = 1;
                            }

                            self.grid[py][px] ^= true;
                        }
                    }
                }

                Ok(())
            }
            // ADD I, Vx
            Instruction(_) if instruction.get_first() == 0xF && instruction.get_kk() == 0x1E => {
                let x = instruction.get_x() as usize;
                let result = self.i as u32 + self.v[x] as u32;
                self.i = result as u16;
                Ok(())
            }
            // LD Vx, [I]
            Instruction(_) if instruction.get_first() == 0xF && instruction.get_kk() == 0x65 => {
                let x = instruction.get_x();
                for i in 0..(x + 1) {
                    self.v[i as usize] = self.memory[self.i as usize + i as usize];
                }
                Ok(())
            }
            // BCD VX
            Instruction(_) if instruction.get_first() == 0xF && instruction.get_kk() == 0x33 => {
                let x = instruction.get_x() as usize;
                let mut value = self.v[x];

                self.memory[(self.i + 2) as usize] = value.rem_euclid(10);
                value /= 10;

                self.memory[(self.i + 1) as usize] = value.rem_euclid(10);
                value /= 10;

                self.memory[self.i as usize] = value.rem_euclid(10);

                Ok(())
            }
            // LD Dt, Vx
            Instruction(_) if instruction.get_first() == 0xF && instruction.get_n() == 0x5 => {
                let x = instruction.get_x() as usize;

                self.dt = self.v[x];

                Ok(())
            }
            // JP addr
            Instruction(_) if instruction.get_first() == 0x1 => {
                self.pc = instruction.get_nnn() as usize;
                Ok(())
            }
            // CALL addr
            Instruction(_) if instruction.get_first() == 0x2 => {
                self.stack[self.sp as usize] = self.pc as u16;
                self.sp += 1;

                let nnn = instruction.get_nnn() as usize;
                self.pc = nnn;
                Ok(())
            }
            // SE Vx, byte
            Instruction(_) if instruction.get_first() == 0x3 => {
                let x = instruction.get_x() as usize;
                if self.v[x] == instruction.get_kk() {
                    self.pc += 2;
                }
                Ok(())
            }
            // SNE Vx, byte
            Instruction(_) if instruction.get_first() == 0x4 => {
                let x = instruction.get_x() as usize;
                if self.v[x] != instruction.get_kk() {
                    self.pc += 2;
                }
                Ok(())
            }
            // SE Vx, Vy
            Instruction(_) if instruction.get_first() == 0x5 && instruction.get_n() == 0x0 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
                Ok(())
            }
            // LD Vx, byte
            Instruction(_) if instruction.get_first() == 0x6 => {
                let x = instruction.get_x() as usize;
                self.v[x] = instruction.get_kk();
                Ok(())
            }
            // ADD Vx, byte
            Instruction(_) if instruction.get_first() == 0x7 => {
                let x = instruction.get_x() as usize;
                let result = (self.v[x] as u16 + instruction.get_kk() as u16) % 256;
                self.v[x] = result as u8;

                Ok(())
            }
            // LD Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x0 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[x] = self.v[y];

                Ok(())
            }
            // OR Vx, Yy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x1 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[x] |= self.v[y];
                Ok(())
            }
            // AND Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x2 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[x] &= self.v[y];
                Ok(())
            }
            // XOR Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x3 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[x] ^= self.v[y];
                Ok(())
            }
            // ADD Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x4 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                let sum = self.v[x] as u16 + self.v[y] as u16;

                self.v[x] = (sum & 0xFF) as u8;
                self.v[0xF] = if sum > 0xFF { 1 } else { 0 };

                Ok(())
            }
            // SUB Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x5 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                let diff = self.v[x] as i16 - self.v[y] as i16;

                self.v[x] = diff.rem_euclid(256) as u8;
                self.v[0xF] = if diff >= 0 { 1 } else { 0 };

                Ok(())
            }
            // SHR Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x6 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[0xF] = self.v[y] & 0x1;
                self.v[x] >>= 1;

                Ok(())
            }
            // SUBN Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0x7 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                let diff = self.v[y] as i16 - self.v[x] as i16;

                self.v[x] = diff.rem_euclid(256) as u8;
                self.v[0xF] = if diff >= 0 { 1 } else { 0 };

                Ok(())
            }
            // SHL Vx, Vy
            Instruction(_) if instruction.get_first() == 0x8 && instruction.get_n() == 0xE => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                self.v[0xF] = self.v[x] >> 7;
                self.v[x] = self.v[y] << 1;

                Ok(())
            }
            // SNE VX, VY
            Instruction(_) if instruction.get_first() == 0x9 && instruction.get_n() == 0x0 => {
                let x = instruction.get_x() as usize;
                let y = instruction.get_y() as usize;

                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }

                Ok(())
            }
            _ => Err(format!("{:04X}: not implemented", instruction).into()),
        }
    }
}

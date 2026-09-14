use std::error::Error;
use std::fs;

use crate::instruction::Instruction;
use crate::io::*;

const START_IND: usize = 0x200;

pub struct Chip8 {
    memory: [u8; 0xFFF + 1],
    v: [u8; 0xF + 1],
    i: u16,
    pc: usize, // original u16
    sp: u8,
    stack: [u16; 16],
    grid: [[bool; 64]; 32],
    io: Box<dyn Io>,
}

impl Chip8 {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut memory = [0; 0xFFF + 1];
        let rom = fs::read(file_path)?;
        memory[START_IND..START_IND + rom.len()].copy_from_slice(&rom);

        Ok(Self {
            memory,
            v: [0; 0xF + 1],
            i: 0,
            pc: START_IND,
            sp: 0,
            stack: [0; 16],
            grid: [[false; 64]; 32],
            io: Box::new(RaylibIo::new()),
        })
    }

    fn print_rom_hex(&mut self) {
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

            self.io.render();
        }
        Ok(())
    }

    fn fetch(&mut self) -> Option<Instruction> {
        let byte_one = self.memory[self.pc];
        self.pc += 1;
        let byte_two = self.memory[self.pc];
        self.pc += 1;

        if byte_one == 0 && byte_two == 0 {
            return None;
        }

        Some(Instruction::new(byte_one, byte_two))
    }

    fn instruct(&mut self, instruction: Instruction) -> Result<(), Box<dyn Error>> {
        match instruction {
            // cls
            Instruction(0x00E0) => {
                self.grid = [[false; 64]; 32];
                Ok(())
            }
            // LD I, adr
            Instruction(_) if instruction.get_first() == 0xA => {
                self.i = instruction.get_nnn();
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
            // JP addr
            Instruction(_) if instruction.get_first() == 0x1 => {
                self.pc = instruction.get_nnn() as usize;
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
            _ => Err(format!("{:04X}: not implemented", instruction).into()),
        }
    }
}

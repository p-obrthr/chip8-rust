use std::error::Error;
use std::fs;

use crate::instruction::Instruction;

const START_IND: usize = 0x200;

pub struct Chip8 {
    memory: [u8; 0xFFF + 1],
    vx: [u8; 0xF],
    pc: usize, // original u16
    sp: u8,
    stack: [u16; 16],
    grid: [[bool; 64]; 32],
}

impl Chip8 {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut memory = [0; 0xFFF + 1];
        let rom = fs::read(file_path)?;
        memory[START_IND..START_IND + rom.len()].copy_from_slice(&rom);

        Ok(Self {
            memory,
            vx: [0; 0xF],
            pc: START_IND,
            sp: 0,
            stack: [0; 16],
            grid: [[false; 64]; 32],
        })
    }

    fn print_rom_hex(&mut self) {
        while let Some(instruction) = self.fetch() {
            println!("{:04X}", instruction);
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(instruction) = self.fetch() {
            self.instruct(instruction)?;
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
            Instruction(0x00E0) => Ok(()),
            _ => Err(format!("{:04X}: not implemented", instruction).into()),
        }
    }
}

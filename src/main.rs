#![allow(dead_code)]
#![allow(unused_imports)]

use std::error::Error;
use std::fmt;

const START_IND: usize = 0x200;

mod chip8;
mod instruction;
use crate::chip8::Chip8;

fn main() -> Result<(), Box<dyn Error>> {
    let mut chip8 = Chip8::new("../ibm.ch8")?;

    //chip8.print_rom_hex();
    chip8.run()?;

    Ok(())
}

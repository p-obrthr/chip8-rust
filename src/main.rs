#![allow(dead_code)]
#![allow(unused_imports)]

use std::error::Error;

mod chip8;
mod instruction;
mod io;

use crate::chip8::Chip8;

fn main() -> Result<(), Box<dyn Error>> {
    let mut chip8 = Chip8::new("../ibm.ch8")?;

    //chip8.print_rom_hex();
    chip8.run()?;

    Ok(())
}

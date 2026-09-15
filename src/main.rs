#![allow(dead_code)]
#![allow(unused_imports)]

use std::env;
use std::error::Error;
use std::fs;

mod chip8;
mod instruction;
mod io;

use crate::chip8::Chip8;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let rom = fs::read(file)?;

    let mut chip8 = Chip8::new(&rom)?;
    chip8.run()?;

    Ok(())
}

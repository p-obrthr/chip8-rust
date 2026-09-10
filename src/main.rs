extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::error::Error;
use std::fs;

use std::time::Duration;

struct Chip8 {
    memory: [u8; 0xFFF + 1],
    vx: [u8; 0xF],
    pc: usize, // original u16
    sp: u8,
    stack: [u16; 16],
    grid: [[bool; 64]; 32],
}

const START_IND: usize = 0x200;

impl Chip8 {
    fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
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

    fn print_rom_hex(&self) {
        for bytes in self.memory[START_IND..].chunks(2) {
            if bytes[0] == 0 && bytes[1] == 0 {
                break;
            }

            println!("{:02X} {:02X}", bytes[0], bytes[1]);
        }
    }

    fn step(&mut self) {
        let inst: Instruction = self.fetch();
        let bytes = inst.get_bytes();

        println!("bytes: {:02X} {:02X}", bytes[0], bytes[1]);

        println!("nnnn:: {:04X}", inst.get_nnn());
    }

    fn fetch(&mut self) -> Instruction {
        let byte_one = self.memory[self.pc];
        self.pc += 1;
        let byte_two = self.memory[self.pc];
        self.pc += 1;

        Instruction::new(byte_one, byte_two)
    }

    // fn instruct(&mut self, instruction: Instruction) -> Result<(), Box<dyn Error>> {
    //     match Instruction

    // }
}

struct Instruction(u16);

impl Instruction {
    fn new(byte_one: u8, byte_two: u8) -> Self {
        let x: u16 = byte_one as u16;
        let y: u16 = byte_two as u16;

        Self((x << 8) + y)
    }

    fn get_bytes(&self) -> [u8; 2] {
        [(self.0 >> 8) as u8, (self.0 & 0x00FF) as u8]
    }

    fn get_nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut chip8 = Chip8::new("../ibm.ch8")?;

    // chip8.print_rom_hex();
    chip8.step();
    chip8.step();
    chip8.step();

    Ok(())

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }
}

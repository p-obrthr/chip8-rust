use std::fmt;

pub struct Instruction(pub u16);

impl fmt::UpperHex for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl Instruction {
    pub fn new(byte_one: u8, byte_two: u8) -> Self {
        let x: u16 = byte_one as u16;
        let y: u16 = byte_two as u16;

        Self((x << 8) + y)
    }

    pub fn get_bytes(&self) -> [u8; 2] {
        [(self.0 >> 8) as u8, (self.0 & 0x00FF) as u8]
    }

    pub fn get_nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

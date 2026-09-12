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

    // .___
    pub fn get_first(&self) -> u8 {
        (self.0 >> 12) as u8
    }

    // _x__
    pub fn get_x(&self) -> u8 {
        (self.0 >> 8) as u8 & 0x0F
    }

    // __y_
    pub fn get_y(&self) -> u8 {
        ((self.0 >> 4) & 0x0F) as u8
    }

    // ___n
    pub fn get_n(&self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    // __kk
    pub fn get_kk(&self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    // _nnn
    pub fn get_nnn(&self) -> u16 {
        self.0 & 0x0FFF
    }
}

use std::fmt;


#[derive(Debug)]
pub struct Color{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}


impl fmt::Display for Color{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:06x}", self.red, self.green, self.blue, (self.red as u32) *65536 + (self.green as u32) * 256 + self.blue as u32)
}
}
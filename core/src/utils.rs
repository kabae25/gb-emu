// Combine two bytes into a u16
pub fn merge_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u8)
}

pub trait byteOps {
    fn high_byte(&self) -> u8;
    fn low_byte(&self) -> u8;
}

impl ByteOps for u16 {
    fn high_byte(&self) -> u8 {
        (self >> 8) as u8
    }
    
    fn low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }
}
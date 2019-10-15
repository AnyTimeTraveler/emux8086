use crate::emux8086::utils::read_u16;
use std::ops::Add;

pub struct Memory {
    data: &'static mut [u8]
}

impl Memory {
    pub fn low_byte(&self) -> u8 {
        self.data[0]
    }
    pub fn high_byte(&self) -> u8 {
        self.data[1]
    }
    pub fn word(&self) -> u8 {
        self.data[1]
    }

}

impl From<Memory> for u16 {
    fn from(reg: Memory) -> Self {
        read_u16(&reg.data, 0)
    }
}

impl From<&mut [u8]> for Memory {
    fn from(data: &mut[u8]) -> Memory {
        Memory {
            data
        }
    }
}


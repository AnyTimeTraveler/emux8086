pub struct Memory {
    raw: &'static mut [u8]
}

pub struct Register {
    raw: &'static mut [u8]
}

trait Readable {
    fn read(&self, pos: usize, length: usize) -> &[u8];
}

trait Writable {
    fn write(&self, src: &[u8], pos: usize);
}

impl Memory {
    pub fn read(&self, pos: usize, length: usize) -> &[u8] {
        &self.raw[pos..pos + length]
    }
    pub fn read_byte_at(&self, pos: usize) -> u8 {
        self.raw[pos]
    }
    pub fn read_word_at(&self, pos: usize) -> u16 {
        ((self.raw[pos] as u16) << 8) | self.raw[pos + 1] as u16
    }
    pub fn read_double_word_at(&self, pos: usize) -> u32 {
        ((self.word_at(pos) as u32) << 16) | self.word_at(pos + 2) as u32
    }
    pub fn read_quad_word_at(&self, pos: usize) -> u64 {
        ((self.double_word_at(pos) as u64) << 32) | self.double_word_at(pos + 4) as u64
    }

    pub fn write(&self, src: &[u8], pos: usize) {
        self.raw[pos..].copy_from_slice(src);
    }
    pub fn write_byte_at(&self, src: u8, pos: usize) {
        self.raw[pos] = src;
    }
}

impl Register {
    pub fn read(&self, pos: usize, length: usize) -> &[u8] {
        &self.raw[pos..pos + length]
    }
    pub fn read_byte_at(&self, pos: usize) -> u8 {
        self.raw[pos]
    }
    pub fn read_low_byte(&self) -> u8 {
        self.raw[0]
    }
    pub fn read_high_byte(&self) -> u8 {
        self.raw[1]
    }
    pub fn read_word(&self) -> u16 {
        ((self.raw[0] as u16) << 8) | self.raw[1] as u16
    }

    pub fn write(&self, src: &[u8], pos: usize) {
        self.raw[pos..].copy_from_slice(src);
    }
    pub fn write_byte_at(&self, src: u8, pos: usize) {
        self.raw[pos] = src;
    }
    pub fn write_high_byte(&self, src: u8) {
        self.raw[1] = src;
    }
    pub fn write_low_byte(&self, src: u8) {
        self.raw[0] = src;
    }
    pub fn write_word(&self, src: u16) {
        self.raw[0] = src as u8;
        self.raw[1] = (src >> 8) as u8;
    }
}

impl From<Register> for u16 {
    fn from(reg: Register) -> Self {
        reg.read_word()
    }
}

impl From<&mut [u8]> for Memory {
    fn from(data: &mut [u8]) -> Memory {
        Memory {
            raw: data
        }
    }
}

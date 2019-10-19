
pub fn read_word(src: &[u8]) -> u16 {
    ((src[1] as u16) << 8) | src[0] as u16
}

pub fn read_double_word(src: &[u8]) -> u32 {
    ((read_word(&src[2..4]) as u32) << 16) | read_word(&src[0..2]) as u32
}

pub fn read_quad_word(src: &[u8]) -> u64 {
    ((read_double_word(&src[4..8]) as u64) << 32) | read_double_word(&src[0..4]) as u64
}

pub fn write_word(dst: &mut [u8], data: u16) {
    for i in 0..2 {
        dst[i] = (data >> (i * 8) as u16) as u8;
    }
}

pub fn write_double_word(dst: &mut [u8], data: u32) {
    for i in 0..4 {
        dst[i] = (data >> (i * 8) as u32) as u8;
    }
}

pub fn write_quad_word(dst: &mut [u8], data: u64) {
    for i in 0..8 {
        dst[i] = (data >> (i * 8) as u64) as u8;
    }
}


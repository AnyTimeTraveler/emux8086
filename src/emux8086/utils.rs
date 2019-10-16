use crate::emux8086::computer::Computer;

pub fn as_u16(data: &[u8], pos: u16) -> u16 {
    ((data[(pos + 1) as usize] as u16) << 8) | data[pos as usize] as u16
}

pub fn read_word(src: &[u8], pos: usize) -> u16 {
    ((src[pos] as u16) << 8) | src[pos + 1] as u16
}
pub fn read_double_word(src: &[u8], pos: usize) -> u32 {
    ((read_word(src, pos) as u32) << 16) | read_word(src, pos + 2) as u32
}
pub fn read_quad_word(src: &[u8], pos: usize) -> u64 {
    ((read_double_word(src, pos) as u64) << 32) | read_double_word(src,pos + 4) as u64
}

pub fn write_word(dst: &mut [u8], pos: usize, data: u16) {
    for i in 0..2 {
        dst[pos + i] = (data >> (i * 8) as u16) as u8;
    }
}

pub fn write_double_word(dst: &mut [u8], pos: usize, data: u32) {
    for i in 0..4 {
        dst[pos + i] = (data >> (i * 8) as u32) as u8;
    }
}

pub fn write_quad_word(dst: &mut [u8], pos: usize, data: u64) {
    for i in 0..8 {
        dst[pos + i] = (data >> (i * 8) as u64) as u8;
    }
}

pub fn u16_as_hex(data: u16) -> String {
    format!("{:01$X}", data, 4)
}

pub fn u8_as_hex(data: u8) -> String {
    format!("{:01$X}", data, 2)
}

pub fn print_registers(computer: &Computer) {
    println!(
        "Registers:\n\
        \tAX {}|{}\n\
        \tBX {}|{}\n\
        \tCX {}|{}\n\
        \tDX {}|{}\n",
        u8_as_hex(*computer.registers.ah), u8_as_hex(*computer.registers.al),
        u8_as_hex(*computer.registers.bh), u8_as_hex(*computer.registers.bl),
        u8_as_hex(*computer.registers.ch), u8_as_hex(*computer.registers.cl),
        u8_as_hex(*computer.registers.dh), u8_as_hex(*computer.registers.dl)
    )
}

pub fn new_print_registers(computer: &Computer) {
    println!(
        "Registers:\n\
        \tAX {:02x}|{:02x}\n\
        \tBX {:02x}|{:02x}\n\
        \tCX {:02x}|{:02x}\n\
        \tDX {:02x}|{:02x}\n",
        *computer.registers.ah, *computer.registers.al,
        *computer.registers.bh, *computer.registers.bl,
        *computer.registers.ch, *computer.registers.cl,
        *computer.registers.dh, *computer.registers.dl
    )
}

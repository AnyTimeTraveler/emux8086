use crate::emux8086::Computer;
use crate::emux8086::register::Memory;

pub fn read_u16(data: &[u8], pos: u16) -> u16 {
    let low_byte = data[pos as usize];
    let high_byte = data[(pos + 1) as usize];
    let mut result = high_byte as u16;
    result = result << 8;
    result = result | low_byte as u16;
    result
}

pub fn mov_reg_to_mem(src: &Memory, src_pos: usize, dst: &mut [u8], dst_pos: usize, count: usize) {
    for i in 0..count {
        dst[dst_pos + i] = src[src_pos + i];
    }
}

pub fn mov_mem_to_reg(src: &[u8], src_pos: u16, dst: &mut [u8], dst_pos: u16, count: u8) {
    let count = count as usize;
    let src_pos = src_pos as usize;
    let dst_pos = dst_pos as usize;

    for i in 0..count {
        dst[dst_pos + i] = src[src_pos + i];
    }
}

pub fn mov(src: &Memory, src_pos: u16, dst: &mut Memory, dst_pos: u16, count: u8) {
    let count = count as usize;
    let src_pos = src_pos as usize;
    let dst_pos = dst_pos as usize;

    for i in 0..count {
        dst[dst_pos + i] = src[src_pos + i];
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
        u8_as_hex(computer.cpu.ax[1]), u8_as_hex(computer.cpu.ax[0]),
        u8_as_hex(computer.cpu.bx[1]), u8_as_hex(computer.cpu.bx[0]),
        u8_as_hex(computer.cpu.cx[1]), u8_as_hex(computer.cpu.cx[0]),
        u8_as_hex(computer.cpu.dx[1]), u8_as_hex(computer.cpu.dx[0])
    )
}

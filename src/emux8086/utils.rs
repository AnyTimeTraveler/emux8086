use crate::emux8086::Computer;
use crate::emux8086::memory::{Memory, Register};

pub fn mov_byte_mem_to_reg(src: &Memory, src_pos: usize, dst: &mut Register, dst_pos: usize) {
    dst.write_byte_at(src[src_pos],dst_pos);
}

pub fn mov_word_mem_to_reg(src: &Memory, src_pos: usize, dst: &mut Register) {
    dst.write(src[src_pos],0);
}

pub fn mov_byte_reg_to_mem(src: &Register, src_pos: usize, dst: &mut Memory, dst_pos: usize) {
    dst.write_byte_at(src.read_byte_at(src_pos), dst_pos);
}

pub fn mov_word_reg_to_mem(src: &Register, dst: &mut Memory, dst_pos: usize) {
    dst.write_byte_at(src.read_low_byte(), dst_pos);
    dst.write_byte_at(src.read_high_byte(), dst_pos+1);
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

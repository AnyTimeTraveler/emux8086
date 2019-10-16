use crate::emux8086::modbyte::AddressingMode::{IndirectAddressingMode, OneByteSignedDisplacement, FourByteSignedDisplacement, RegisterAddressingMode};
use crate::emux8086::CPU;

enum AddressingMode {
    IndirectAddressingMode,
    OneByteSignedDisplacement,
    FourByteSignedDisplacement,
    RegisterAddressingMode,
}

pub fn decode(byte: u8, cpu: &CPU) {
    let mod_bits = byte & 0b11000000;
    let mode = match mod_bits {
        0b11000000 => RegisterAddressingMode,
        0b10000000 => FourByteSignedDisplacement,
        0b01000000 => OneByteSignedDisplacement,
        0b00000000 => IndirectAddressingMode,
        _ => panic!("Logic error!")
    };

    let reg = byte & 0b00111000;

    let bits = 8;

    let mut register: &mut u8;
    if bits == 8 {
        register = match reg {
            0b00000000 => cpu.ax.point_to_low_byte(),
            0b00001000 => cpu.cx.point_to_low_byte(),
            0b00010000 => cpu.dx.point_to_low_byte(),
            0b00011000 => cpu.bx.point_to_low_byte(),
            0b00100000 => cpu.ax.point_to_high_byte(),
            0b00101000 => cpu.cx.point_to_high_byte(),
            0b00110000 => cpu.dx.point_to_high_byte(),
            0b00111000 => cpu.bx.point_to_high_byte(),
            _ => panic!("Logic error!")
        };
    } else {
        register = match reg {
            0b00000000 => cpu.ax,
            0b00001000 => cpu.cx,
            0b00010000 => cpu.dx,
            0b00011000 => cpu.bx,
            0b00100000 => cpu.sp,
            0b00101000 => cpu.bp,
            0b00110000 => cpu.si,
            0b00111000 => cpu.di,
            _ => panic!("Logic error!")
        };
    }
    let register = register;
}
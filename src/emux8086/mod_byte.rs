use crate::emux8086::mod_byte::AddressingMode::{FourByteSignedDisplacement, IndirectAddressingMode, OneByteSignedDisplacement, RegisterAddressingMode};
use crate::emux8086::mod_byte::InstructionWidth::{EightBits, SixteenBits};
use crate::emux8086::registers::Registers;

#[derive(PartialEq)]
pub enum AddressingMode {
    IndirectAddressingMode,
    OneByteSignedDisplacement,
    FourByteSignedDisplacement,
    RegisterAddressingMode,
}

#[derive(PartialEq)]
pub enum InstructionWidth {
    EightBits,
    SixteenBits,
    ThirtyTwoBits,
}

pub fn read_mode(byte: u8) -> AddressingMode {
    match byte & 0b11000000 {
        0b11000000 => RegisterAddressingMode,
        0b10000000 => FourByteSignedDisplacement,
        0b01000000 => OneByteSignedDisplacement,
        0b00000000 => IndirectAddressingMode,
        _ => panic!("Logic error!")
    }
}

pub fn read_reg(byte:u8, registers: &Registers, width: InstructionWidth) -> usize{
    let reg_bits = byte & 0b00111000;
    if width == EightBits {
        match reg_bits {
            0b00000000 => registers.al,
            0b00001000 => registers.cl,
            0b00010000 => registers.dl,
            0b00011000 => registers.bl,
            0b00100000 => registers.ah,
            0b00101000 => registers.ch,
            0b00110000 => registers.dh,
            0b00111000 => registers.bh,
            _ => panic!("Logic error!")
        }
    } else if width == SixteenBits {
        match reg_bits {
            0b00000000 => registers.ax,
            0b00001000 => registers.cx,
            0b00010000 => registers.dx,
            0b00011000 => registers.bx,
            0b00100000 => registers.sp,
            0b00101000 => registers.bp,
            0b00110000 => registers.si,
            0b00111000 => registers.di,
            _ => panic!("Logic error!")
        }
    } else {
        panic!("Logic error!")
    }
}

use crate::emux8086::mod_byte::AddressingMode::{FourByteSignedDisplacement, IndirectAddressingMode, OneByteSignedDisplacement, RegisterAddressingMode};
use crate::emux8086::mod_byte::InstructionWidth::{EightBits, SixteenBits};
use crate::emux8086::registers::Registers;
use crate::emux8086::utils::as_u16;

#[derive(PartialEq)]
enum AddressingMode {
    IndirectAddressingMode,
    OneByteSignedDisplacement,
    FourByteSignedDisplacement,
    RegisterAddressingMode,
}

#[derive(PartialEq)]
enum InstructionWidth {
    EightBits,
    SixteenBits,
    ThirtyTwoBits,
}

pub fn decode_mod_byte(byte: u8, registers: &Registers, width: InstructionWidth) {
    let mod_bits = byte & 0b11000000;
    let mode = match mod_bits {
        0b11000000 => RegisterAddressingMode,
        0b10000000 => FourByteSignedDisplacement,
        0b01000000 => OneByteSignedDisplacement,
        0b00000000 => IndirectAddressingMode,
        _ => panic!("Logic error!")
    };

    let reg = byte & 0b00111000;

    let mut register = if width == EightBits {
        // TODO: Replace by formula using registers.raw ?
        match reg {
            0b00000000 => &mut registers.ax[0..1],
            0b00001000 => &mut registers.cx[0..1],
            0b00010000 => &mut registers.dx[0..1],
            0b00011000 => &mut registers.bx[0..1],
            0b00100000 => &mut registers.ax[1..2],
            0b00101000 => &mut registers.cx[1..2],
            0b00110000 => &mut registers.dx[1..2],
            0b00111000 => &mut registers.bx[1..2],
            _ => panic!("Logic error!")
        }
    } else if width == SixteenBits {
        // TODO: Replace by formula using registers.raw ?
        match reg {
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
        unimplemented!();
    };

    println!("{:02x}",as_u16(register,0));
}

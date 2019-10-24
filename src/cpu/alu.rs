use std::num::Wrapping;
use std::u8;

pub fn add(src: &[u8], dst: &mut [u8]) {
    let mut previous_overflow = false;
    for i in 0..dst.len() {
        let operand = Wrapping(src[i]) + Wrapping(if previous_overflow { 1 } else { 0 });
        let (result, overflow) = dst[i].overflowing_add(operand.0);
        dst[i] = result;
        previous_overflow = overflow;
    }
}

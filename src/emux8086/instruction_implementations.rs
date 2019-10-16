pub fn mov(src: &[u8], src_pos: u16, dst: &mut [u8], dst_pos: u16, count: u8) {
    for i in 0..count as u16 {
        dst[dst_pos as usize + 1] = src[(src_pos + i) as usize];
    }
}
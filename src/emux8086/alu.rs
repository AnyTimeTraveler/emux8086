
pub fn add(src: &[u8], dst: &mut [u8]) {
    let mut overflow = false;
    for i in 0..dst.len() {
        if overflow {
            dst[i] += 1;
        }
        overflow = match dst[i].checked_add(src[i]) {
            Some(_) => false,
            None => true,
        };
        dst[i] = dst[i] + src[i];
    }
}
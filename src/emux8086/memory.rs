pub struct Memory<'a> {
    pub program_memory: &'a mut [u8],
    pub stack: &'a mut [u8],
    pub interrupts: &'a mut [u8],
    pub debugger: &'a mut [u8],
    pub raw: &'a mut [u8; 1_048_576],
}

impl <'a> Memory<'a>{
    pub fn new() -> Memory<'a> {
        let mut data = [0u8;1_048_576];
        Memory {
            program_memory: &mut data[0x100..],
            stack: &mut data[1..4],
            interrupts: &mut data[1..4],
            debugger: &mut data[1..4],
            raw: &mut data,
        }
    }
}

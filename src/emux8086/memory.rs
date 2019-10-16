pub struct Memory {
    pub program_memory: &'static mut [u8],
    pub stack: &'static mut [u8],
    pub interrupts: &'static mut [u8],
    pub debugger: &'static mut [u8],
    pub raw: [u8; 1_048_576],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            program_memory: &mut [],
            stack: &mut [],
            interrupts: &mut [],
            debugger: &mut raw[100..200],
            raw: [0u8;1_048_576],
        }
    }
}

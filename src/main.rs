mod cpu;
mod peripherals;

#[cfg(test)]
mod tests;


fn main() {
    tui::start().expect("TUI Error");
//    let mut file = File::open("asm/copy").expect("Error opening file!");

//    let mut computer = Computer::new();

//    let mut program_data = [0u8; 1024];

//    let read = file.read(&mut program_data).expect("Error reading file!");

//    println!("{} bytes loaded!", read);

//    computer.load_program(&program_data);

//            computer.step();
//        print_registers(&computer);
}

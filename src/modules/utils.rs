use crate::modules::vm::Vm;
use std::time::Duration;

pub fn center_print(text: &str, total_width: usize) {
    let padding = (total_width - text.len()) / 2;
    println!("{} {} {}", "-".repeat(padding), text, "-".repeat(total_width - padding - text.len()));
}

pub fn debug(elapsed: Duration, vm: &Vm) {
    center_print("DEBUG INFO", 80);
    println!("Registers: {:?}", vm.reg);
    println!("Memory: {:?}", vm.mem);
    println!("Program Counter: {:?}", vm.pc);
    println!("Zero Flag: {:?}", vm.zf);
    println!("Program: {:?}", vm.program);
    println!("Program Length: {:?}", vm.program.len());
    println!("Execution time: {:?}", elapsed);
}

pub fn parse_args() -> Vec<String> {
    std::env::args().collect()
}
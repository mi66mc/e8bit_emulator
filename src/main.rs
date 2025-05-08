#[derive(Debug)]
struct Vm {
    pc: usize,
    reg: [u8; 4],
    mem: [u8; 256],
    program: Vec<Instruction>,
    zf: bool,
}

#[derive(Debug, Clone, Copy)]
enum Reg {
    A,
    B,
    C,
    D
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    MOV(Reg, Source),
    STORE(Reg, MemSrc),
    ADD(Reg, Source),
    SUB(Reg, Source),
    MUL(Reg, Source),
    DIV(Reg, Source),
    JMP(usize),
    JZ(usize),
    JNZ(usize),
    LOOP(usize, Reg),
    PRINT(Reg, bool),
    PRINTCH(Reg, bool),
    INPUT(Reg),
    HALT
}

#[derive(Debug, Clone, Copy)]
enum Source {
    Reg(Reg),
    Mem(MemSrc),
    Lit(u8),
}

#[derive(Debug, Clone, Copy)]
enum MemSrc {
    Reg(Reg),
    Addr(u8)
}

impl Vm {
    fn new() -> Self {
        Vm {
            pc: 0,
            reg: [0; 4],
            mem: [0; 256],
            program: Vec::new(),
            zf: false,
        }
    }

    fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    fn reg_index(&self, reg: Reg) -> usize {
        match reg {
            Reg::A => 0,
            Reg::B => 1,
            Reg::C => 2,
            Reg::D => 3,
        }
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            let instruction = &self.program[self.pc];
            match instruction {
                Instruction::MOV(reg, src) => self.mov(*reg, *src),
                Instruction::STORE(reg, mem_src) => self.store(*reg, *mem_src),
                Instruction::ADD(reg, src) => self.add(*reg, *src),
                Instruction::SUB(reg, src) => self.sub(*reg, *src),
                Instruction::MUL(reg, src) => self.mul(*reg, *src),
                Instruction::DIV(reg, src) => self.div(*reg, *src),
                Instruction::JMP(addr) => { self.jmp(*addr); continue; },
                Instruction::JZ(addr) => { self.jz(*addr); continue; },
                Instruction::JNZ(addr) => { self.jnz(*addr); continue; },
                Instruction::LOOP(addr, reg) => { self.loop_fn(*addr, *reg); continue; },
                Instruction::PRINT(reg, opt) => self.print(*reg, *opt),
                Instruction::PRINTCH(reg, opt) => self.printch(*reg, *opt),
                Instruction::INPUT(reg) => self.input(*reg),
                Instruction::HALT => {
                    println!("\n!-!- HALT !-!\n");
                    break;
                }
            }
            self.pc += 1;
        }
    }

    fn mov(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                self.reg[self.reg_index(reg)] = self.reg[src_index];
                self.zf =
                    self.reg[src_index] == 0;
            }
            Source::Mem(value) => {
                let mem_value = self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ];
                self.reg[self.reg_index(reg)] = mem_value;
                self.zf =
                    mem_value == 0;
            }
            Source::Lit(value) => {
                self.reg[self.reg_index(reg)] = value;
                self.zf =
                    value == 0;
            }
        }
        // println!("MOV {:?} {:?}", reg, src);
    }

    fn store(&mut self, reg: Reg, mem_src: MemSrc) {
        match mem_src {
            MemSrc::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                self.mem[self.reg[src_index] as usize] = self.reg[self.reg_index(reg)];
            }
            MemSrc::Addr(addr) => {
                self.mem[addr as usize] = self.reg[self.reg_index(reg)];
            }
        }

        // println!("STORE {:?} {:?}", reg, src);
    }

    fn add(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                let r = self.reg[self.reg_index(reg)].wrapping_add(self.reg[src_index]);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Mem(value) => {
                let v = self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ];
                let r = self.reg[self.reg_index(reg)].wrapping_add(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Lit(value) => {
                let v = value;
                let r = self.reg[self.reg_index(reg)].wrapping_add(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
        }

        // println!("ADD {:?} {:?}", reg, src);
    }

    fn sub(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                let r = self.reg[self.reg_index(reg)].wrapping_sub(self.reg[src_index]);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Mem(value) => {
                let v = self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ];
                let r = self.reg[self.reg_index(reg)].wrapping_sub(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Lit(value) => {
                let v = value;
                let r = self.reg[self.reg_index(reg)].wrapping_sub(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
        }

        // println!("SUB {:?} {:?}", reg, src);
    }
    
    fn div(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                if self.reg[self.reg_index(src_reg)] == 0 {
                    panic!("Division by zero");
                }
                let src_index = self.reg_index(src_reg);
                let r = self.reg[self.reg_index(reg)].wrapping_div(self.reg[src_index]);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Mem(value) => {
                if self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ] == 0 {
                    panic!("Division by zero");
                }
                let v = self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ];
                let r = self.reg[self.reg_index(reg)].wrapping_div(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Lit(value) => {
                if value == 0 {
                    panic!("Division by zero");
                }
                let v = value;
                let r = self.reg[self.reg_index(reg)].wrapping_div(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
        }

        // println!("DIV {:?} {:?}", reg, src);
    }

    fn mul(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                let r = self.reg[self.reg_index(reg)].wrapping_mul(self.reg[src_index]);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Mem(value) => {
                let v = self.mem[
                    match value {
                        MemSrc::Reg(src_reg) => self.reg[self.reg_index(src_reg)] as usize,
                        MemSrc::Addr(addr) => addr as usize,
                    }
                ];
                let r = self.reg[self.reg_index(reg)].wrapping_mul(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
            Source::Lit(value) => {
                let v = value;
                let r = self.reg[self.reg_index(reg)].wrapping_mul(v);
                self.zf =
                    r == 0;
                self.reg[self.reg_index(reg)] = r;
            }
        }

        // println!("MUL {:?} {:?}", reg, src);
    }

    fn jmp(&mut self, addr: usize) {
        self.pc = addr;
        // println!("JMP {:?}", addr);
    }

    fn jz(&mut self, addr: usize) {
        if self.zf {
            self.pc = addr;
        } else {
            self.pc += 1;
        }
        // println!("JZ {:?}", addr);
    }

    fn jnz(&mut self, addr: usize) {
        if !self.zf {
            self.pc = addr;
        } else {
            self.pc += 1;
        }
        // println!("JNZ {:?}", addr);
    }

    fn loop_fn(&mut self, addr: usize, reg: Reg) {
        let index = self.reg_index(reg);
        if self.reg[index] > 0 {
            self.pc = addr; // Jump if not zero
        } else {
            self.pc += 1; // End loop
        }
        // println!("LOOP {:?} {:?}", addr, reg);
    }

    fn print(&mut self, reg: Reg, opt: bool) {
        let val = self.reg[self.reg_index(reg)];
        if opt {
            println!("{}", val);
        } else {
            print!("{}", val);
            use std::io::{stdout, Write};
            let _ = stdout().flush();
        }
    }

    fn printch(&mut self, reg: Reg, opt: bool) {
        let val = self.reg[self.reg_index(reg)];
        if opt {
            println!("{}", val as char);
        } else {
            print!("{}", val as char);
            use std::io::{stdout, Write};
            let _ = stdout().flush();
        }
    }

    fn input(&mut self, reg: Reg) {
        use std::io::{stdout, Write};
        let mut input = String::new();
        print!("INPUT {:?}: ", reg);
        let _ = stdout().flush();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        let trimmed = input.trim();
    
        let value = if let Ok(num) = trimmed.parse::<u8>() {
            num
        } else if trimmed.len() == 1 {
            trimmed.chars().next().unwrap() as u8
        } else if trimmed.len() == 0 {
            self.zf = true;
            0
        } else {
            panic!("Invalid input: expected a number or single character");
        };
    
        self.reg[self.reg_index(reg)] = value;
    }
    
    
}

fn parse_args() -> Vec<String> {
    std::env::args().collect()
}

fn parse_program(file_path: Option<&str>) -> Vec<Instruction> {
    if let Some(path) = file_path {
        let content = std::fs::read_to_string(path).expect("Failed to read file");
        content
            .lines()
            .flat_map(|line| {
                let line = line.trim();
                let line = if let Some(comment_start) = line.find("//") {
                    &line[..comment_start].trim()
                } else {
                    line
                };

                line.split(';').filter_map(|segment| {
                    let segment = segment.trim();
                    if segment.is_empty() {
                        return None;
                    }

                    let parts: Vec<&str> = segment.split_whitespace().collect();

                    match parts.as_slice() {
                        ["MOV", reg, src] => Some(Instruction::MOV(parse_reg(reg), parse_source(src))),
                        ["PRINT", reg] => Some(Instruction::PRINT(parse_reg(reg), true)),
                        ["PRINT", reg, opt] if *opt == "-N" => Some(Instruction::PRINT(parse_reg(reg), false)),
                        ["PRINTCH", reg] => Some(Instruction::PRINTCH(parse_reg(reg), true)),
                        ["PRINTCH", reg, opt] if *opt == "-N" => Some(Instruction::PRINTCH(parse_reg(reg), false)),
                        ["ADD", reg, src] => Some(Instruction::ADD(parse_reg(reg), parse_source(src))),
                        ["SUB", reg, src] => Some(Instruction::SUB(parse_reg(reg), parse_source(src))),
                        ["MUL", reg, src] => Some(Instruction::MUL(parse_reg(reg), parse_source(src))),
                        ["DIV", reg, src] => Some(Instruction::DIV(parse_reg(reg), parse_source(src))),
                        ["STORE", reg, src] => Some(Instruction::STORE(parse_reg(reg), parse_mem_src(src))),
                        ["JMP", addr] => Some(Instruction::JMP(addr.parse().unwrap())),
                        ["JZ", addr] => Some(Instruction::JZ(addr.parse().unwrap())),
                        ["JNZ", addr] => Some(Instruction::JNZ(addr.parse().unwrap())),
                        ["LOOP", addr, reg] => Some(Instruction::LOOP(addr.parse().unwrap(), parse_reg(reg))),
                        ["INPUT", reg] => Some(Instruction::INPUT(parse_reg(reg))),
                        ["HALT"] => Some(Instruction::HALT),
                        _ => panic!("Unknown instruction: {}", segment),
                    }
                })
            })
            .collect()
    } else {
        vec![Instruction::HALT]
    }
}


fn parse_reg(reg: &str) -> Reg {
    match reg {
        "A" => Reg::A,
        "B" => Reg::B,
        "C" => Reg::C,
        "D" => Reg::D,
        _ => panic!("Unknown register: {}", reg),
    }
}

fn parse_source(src: &str) -> Source {
    if let Ok(lit) = src.parse::<u8>() {
        Source::Lit(lit)
    } else if src.starts_with("'") && src.ends_with("'") && src.len() == 3 {
        let char_value = src.chars().nth(1).unwrap() as u8;
        Source::Lit(char_value)
    } else if src.starts_with("[") && src.ends_with(']') {
        let inner = &src[1..src.len() - 1];
        if let Ok(addr) = inner.parse::<u8>() {
            Source::Mem(MemSrc::Addr(addr))
        } else if inner.chars().all(|c| c.is_alphabetic()) {
            Source::Mem(MemSrc::Reg(parse_reg(inner)))
        } else {
            panic!("Invalid memory source: {}", src);
        }
    } else {
        Source::Reg(parse_reg(src))
    }
}

fn parse_mem_src(src: &str) -> MemSrc {
    if src.starts_with("[") && src.ends_with(']') {
        if src[1..src.len() - 1].chars().all(|c| c.is_alphabetic()) {
            let reg = parse_reg(&src[1..src.len() - 1]);
            return MemSrc::Reg(reg);
        }
        let addr = src[1..src.len() - 1].parse::<u8>().expect("Expected a numeric memory address inside brackets");
        MemSrc::Addr(addr)
    } else {
        panic!("Invalid memory source: {}", src);
    }
}

fn main() {
    let args = parse_args();
    let file_path = args.get(1).map(|s| s.as_str());
    let program = parse_program(file_path);
    let mut vm = Vm::new();
    vm.load_program(program);
    vm.run();
    println!("---------------------------------------------------\nExecution finished.\n---------------------------------------------------");
    println!("Registers: {:?}", vm.reg);
    println!("Memory: {:?}", vm.mem);
    println!("Program Counter: {:?}", vm.pc);
    println!("Zero Flag: {:?}", vm.zf);
    println!("Program: {:?}", vm.program);
    println!("Program Length: {:?}", vm.program.len());
}

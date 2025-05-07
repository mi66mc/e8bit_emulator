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
    STORE(Reg, u8),
    ADD(Reg, Source),
    SUB(Reg, Source),
    MUL(Reg, Source),
    DIV(Reg, Source),
    JMP(usize),
    JZ(usize),
    JNZ(usize),
    LOOP(usize, Reg),
    PRINT(Reg),
    HALT
}

#[derive(Debug, Clone, Copy)]
enum Source {
    Reg(Reg),
    Mem(u8),
    Lit(u8),
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
                Instruction::STORE(reg, src) => self.store(*reg, *src),
                Instruction::ADD(reg, src) => self.add(*reg, *src),
                Instruction::SUB(reg, src) => self.sub(*reg, *src),
                Instruction::MUL(reg, src) => self.mul(*reg, *src),
                Instruction::DIV(reg, src) => self.div(*reg, *src),
                Instruction::JMP(addr) => self.jmp(*addr),
                Instruction::JZ(addr) => self.jz(*addr),
                Instruction::JNZ(addr) => self.jnz(*addr),
                Instruction::LOOP(addr, reg) => { self.loop_fn(*addr, *reg); continue; },
                Instruction::PRINT(reg) => self.print(*reg),
                Instruction::HALT => {
                    println!("!-!- HALT !-!");
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
                let mem_value = self.mem[value as usize];
                self.reg[self.reg_index(reg)] = mem_value;
                self.zf =
                    value == 0;
            }
            Source::Lit(value) => {
                self.reg[self.reg_index(reg)] = value;
                self.zf =
                    value == 0;
            }
        }
        // println!("MOV {:?} {:?}", reg, src);
    }

    fn store(&mut self, reg: Reg, src: u8) {
        let index = self.reg_index(reg);
        self.mem[src as usize] = self.reg[index];

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
                let v = self.mem[value as usize];
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
                let v = self.mem[value as usize];
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
                if self.mem[value as usize] == 0 {
                    panic!("Division by zero");
                }
                let v = self.mem[value as usize];
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
                let v = self.mem[value as usize];
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
        }
        // println!("JZ {:?}", addr);
    }

    fn jnz(&mut self, addr: usize) {
        if !self.zf {
            self.pc = addr;
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

    fn print(&mut self, reg: Reg) {
        println!("{}", self.reg[self.reg_index(reg)]);
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
            .filter_map(|line| {
                let line = line.trim();
                let line = if let Some(comment_start) = line.find("//") {
                    &line[..comment_start].trim()
                } else {
                    line
                };
                if line.is_empty() {
                    return None;
                }
                line.split(';')
                    .filter_map(|segment| {
                        let segment = segment.trim();
                        if segment.is_empty() {
                            return None;
                        }
                        let parts: Vec<&str> = segment.split_whitespace().collect();
                        match parts.as_slice() {
                            ["MOV", reg, src] => Some(Instruction::MOV(parse_reg(reg), parse_source(src))),
                            ["PRINT", reg] => Some(Instruction::PRINT(parse_reg(reg))),
                            ["ADD", reg, src] => Some(Instruction::ADD(parse_reg(reg), parse_source(src))),
                            ["SUB", reg, src] => Some(Instruction::SUB(parse_reg(reg), parse_source(src))),
                            ["MUL", reg, src] => Some(Instruction::MUL(parse_reg(reg), parse_source(src))),
                            ["DIV", reg, src] => Some(Instruction::DIV(parse_reg(reg), parse_source(src))),
                            ["STORE", reg, addr] => Some(Instruction::STORE(parse_reg(reg), addr.parse().unwrap())),
                            ["JMP", addr] => Some(Instruction::JMP(addr.parse().unwrap())),
                            ["JZ", addr] => Some(Instruction::JZ(addr.parse().unwrap())),
                            ["JNZ", addr] => Some(Instruction::JNZ(addr.parse().unwrap())),
                            ["LOOP", addr, reg] => Some(Instruction::LOOP(addr.parse().unwrap(), parse_reg(reg))),
                            ["HALT"] => Some(Instruction::HALT),
                            _ => panic!("Unknown instruction: {}", segment),
                        }
                    })
                    .next()
            })
            .collect()
    } else {
        vec![
            Instruction::HALT,                              // Stop the program
        ]
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
    } else if src.starts_with("[") && src.ends_with(']') {
        let addr = src[1..src.len() - 1].parse::<u8>().expect("Invalid memory address");
        Source::Mem(addr)
    } else {
        Source::Reg(parse_reg(src))
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

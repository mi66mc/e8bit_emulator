use std::result;

#[derive(Debug)]
struct vm {
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
    PRINT(Reg),
    HALT
}

#[derive(Debug, Clone, Copy)]
enum Source {
    Reg(Reg),
    Mem(u8),
    Lit(u8),
}

impl vm {
    fn new() -> Self {
        vm {
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

    fn print(&mut self, reg: Reg) {
        println!("{}", self.reg[self.reg_index(reg)]);
    }
    
}

fn main() {
    let mut vm = vm::new();
    let program = vec![
        Instruction::MOV(Reg::A, Source::Lit(60)),      // Initialize A with 60
        Instruction::MOV(Reg::B, Source::Lit(10)),      // Initialize B with 10
        Instruction::MOV(Reg::C, Source::Lit(5)),       // Set loop counter to 5
        Instruction::SUB(Reg::A, Source::Reg(Reg::B)),  // Subtract B from A
        Instruction::PRINT(Reg::A),                     // Print A
        Instruction::SUB(Reg::C, Source::Lit(1)),       // Decrement loop counter
        Instruction::JNZ(2),                            // Jump back if C != 0
        Instruction::HALT,                              // Stop the program
    ];
    vm.load_program(program);
    vm.run();
    println!("Final state: {:?}", vm);
}

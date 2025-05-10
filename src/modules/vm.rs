use std::time::Duration;
use std::io::{stdout, Write};

#[derive(Debug)]
pub struct Vm {
    pub pc: u16,
    pub reg: [u8; 5],
    pub mem: [u8; 256],
    pub program: Vec<Instruction>,
    pub zf: bool,
    pub screen: [[char; 80]; 25],
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    MOV(Reg, Source),
    STORE(Reg, MemSrc),
    ADD(Reg, Source),
    SUB(Reg, Source),
    MUL(Reg, Source),
    MULH(Reg, Reg, Reg),
    DIV(Reg, Source),
    JMP(usize),
    JZ(usize),
    JNZ(usize),
    LOOP(usize, Reg),
    PRINT(Reg, bool),
    PRINTCH(Reg, bool),
    INPUT(Reg),
    DRAW(Source, Source, Source),
    SLP(usize),
    CLS,
    RENDER,
    HALT
}

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Reg(Reg),
    Mem(MemSrc),
    Lit(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum MemSrc {
    Reg(Reg),
    Addr(u8)
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            pc: 0,
            reg: [0; 5],
            mem: [0; 256],
            program: Vec::new(),
            zf: false,
            screen: [[' '; 80]; 25],
        }
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.program = program;
    }

    pub fn reg_index(&self, reg: Reg) -> usize {
        match reg {
            Reg::A => 0,
            Reg::B => 1,
            Reg::C => 2,
            Reg::D => 3,
            Reg::E => 4,
        }
    }

    pub fn run(&mut self) {
        while self.pc < self.program.len() as u16 {
            let instruction = &self.program[self.pc as usize];
            match instruction {
                Instruction::MOV(reg, src) => self.mov(*reg, *src),
                Instruction::STORE(reg, mem_src) => self.store(*reg, *mem_src),
                Instruction::ADD(reg, src) => self.add(*reg, *src),
                Instruction::SUB(reg, src) => self.sub(*reg, *src),
                Instruction::MUL(reg, src) => self.mul(*reg, *src),
                Instruction::MULH(dest, src1, src2) => self.mulh(*dest, *src1, *src2),
                Instruction::DIV(reg, src) => self.div(*reg, *src),
                Instruction::JMP(addr) => { self.jmp(*addr); continue; },
                Instruction::JZ(addr) => { self.jz(*addr); continue; },
                Instruction::JNZ(addr) => { self.jnz(*addr); continue; },
                Instruction::LOOP(addr, reg) => { self.loop_fn(*addr, *reg); continue; },
                Instruction::PRINT(reg, opt) => self.print(*reg, *opt),
                Instruction::PRINTCH(reg, opt) => self.printch(*reg, *opt),
                Instruction::INPUT(reg) => self.input(*reg),
                Instruction::DRAW(x, y, src) => self.draw(*x, *y, *src),
                Instruction::SLP(dur) => { self.sleep(*dur); },
                Instruction::CLS => self.cls(),
                Instruction::RENDER => self.render_screen(),
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

    fn mulh(&mut self, dest: Reg, src1: Reg, src2: Reg) {
        let src1_val = self.reg[self.reg_index(src1)] as u16;
        let src2_val = self.reg[self.reg_index(src2)] as u16;
        let product = src1_val * src2_val;
        self.reg[self.reg_index(dest)] = (product >> 8) as u8; // high byte
        self.zf = (product >> 8) == 0; // set zero flag if high byte is zero
    }

    fn jmp(&mut self, addr: usize) {
        self.pc = addr as u16;
        // println!("JMP {:?}", addr);
    }

    fn jz(&mut self, addr: usize) {
        if self.zf {
            self.pc = addr as u16;
        } else {
            self.pc += 1;
        }
        // println!("JZ {:?}", addr);
    }

    fn jnz(&mut self, addr: usize) {
        if !self.zf {
            self.pc = addr as u16;
        } else {
            self.pc += 1;
        }
        // println!("JNZ {:?}", addr);
    }

    fn loop_fn(&mut self, addr: usize, reg: Reg) {
        let index = self.reg_index(reg);
        if self.reg[index] > 0 {
            self.pc = addr as u16; // Jump if not zero
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
            let _ = stdout().flush();
        }
    }

    fn printch(&mut self, reg: Reg, opt: bool) {
        let val = self.reg[self.reg_index(reg)];
        if opt {
            println!("{}", val as char);
        } else {
            print!("{}", val as char);
            let _ = stdout().flush();
        }
    }

    fn input(&mut self, reg: Reg) {
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

    fn draw(&mut self, x: Source, y: Source, src: Source) {
        let x_val = match x {
            Source::Lit(val) => val,
            Source::Reg(reg) => self.reg[self.reg_index(reg)],
            Source::Mem(mem_src) => match mem_src {
                MemSrc::Addr(addr) => self.mem[addr as usize],
                MemSrc::Reg(reg) => self.reg[self.reg_index(reg)],
            },
        };

        let y_val = match y {
            Source::Lit(val) => val,
            Source::Reg(reg) => self.reg[self.reg_index(reg)],
            Source::Mem(mem_src) => match mem_src {
                MemSrc::Addr(addr) => self.mem[addr as usize],
                MemSrc::Reg(reg) => self.reg[self.reg_index(reg)],
            },
        };

        let char_val = match src {
            Source::Lit(val) => val as char,
            Source::Reg(reg) => self.reg[self.reg_index(reg)] as char,
            Source::Mem(mem_src) => match mem_src {
                MemSrc::Addr(addr) => self.mem[addr as usize] as char,
                MemSrc::Reg(reg) => self.reg[self.reg_index(reg)] as char,
            },
        };

        // if src == 0 turn into 32
        let char_val = if char_val == 0 as char { ' ' } else { char_val };

        if x_val < 80 && y_val < 25 {
            self.screen[y_val as usize][x_val as usize] = char_val;
        }
    }

    fn cls(&mut self) {
        self.screen = [[' '; 80]; 25];
    }
    
    fn render_screen(&self) {
        println!("{}", format!("+{}+", "-".repeat(80)));
        for row in self.screen.iter() {
            print!("|");
            for &ch in row.iter() {
                print!("{}", ch);
            }
            println!("|");
        }
        println!("{}", format!("+{}+", "-".repeat(80)));
    }

    fn sleep(&self, duration: usize) {
        std::thread::sleep(Duration::from_millis(duration as u64));
    }
    
}
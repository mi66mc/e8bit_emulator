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
                _ => {}
            }
            // match instruction {
            //     Instruction::MOV(reg, src) => self.mov(*reg, *src),
            //     Instruction::ADD(reg, src) => self.add(*reg, *src),
            //     Instruction::SUB(reg, src) => self.sub(*reg, *src),
            //     Instruction::MUL(reg, src) => self.mul(*reg, *src),
            //     Instruction::DIV(reg, src) => self.div(*reg, *src),
            //     Instruction::JMP(addr) => self.jmp(*addr),
            //     Instruction::JZ(addr) => self.jz(*addr),
            //     Instruction::JNZ(addr) => self.jnz(*addr),
            //     Instruction::PRINT(reg) => self.print(*reg),
            // }
            self.pc += 1;
        }
    }

    fn mov(&mut self, reg: Reg, src: Source) {
        match src {
            Source::Reg(src_reg) => {
                let src_index = self.reg_index(src_reg);
                self.reg[self.reg_index(reg)] = self.reg[src_index];
            }
            Source::Mem(value) => {
                let mem_value = self.mem[value as usize];
                self.reg[self.reg_index(reg)] = mem_value;
            }
            Source::Lit(value) => {
                self.reg[self.reg_index(reg)] = value;
            }
        }
        println!("MOV {:?} {:?}", reg, src);
    }

    fn store(&mut self, reg: Reg, src: u8) {
        let index = self.reg_index(reg);
        self.mem[src as usize] = self.reg[index];

        println!("STORE {:?} {:?}", reg, src);
    }

    fn add(&mut self, reg: Reg, src: Source) {
        
    }
    
}

fn main() {
    let mut vm = vm::new();
    let program = vec![
        Instruction::MOV(Reg::A, Source::Lit(5)),
        Instruction::MOV(Reg::B, Source::Lit(10)),
        Instruction::STORE(Reg::A, 0),
        Instruction::MOV(Reg::A, Source::Mem(0)),
        Instruction::ADD(Reg::A, Source::Reg(Reg::B)),
        Instruction::PRINT(Reg::A),
    ];
    vm.load_program(program);
    vm.run();
    println!("Final state: {:?}", vm);
}

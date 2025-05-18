use std::io::{stdout, Write};
use crate::modules::utils::center_print;
use crate::modules::vm::{Instruction, Reg, Source, MemSrc};

fn parse_label_or_addr(addr: &str, label_map: &std::collections::HashMap<String, usize>) -> usize {
    if let Ok(num) = addr.parse::<usize>() {
        num
    } else {
        *label_map.get(addr).unwrap_or_else(|| panic!("Unknown label: {}", addr))
    }
}

fn parse_instruction(parts: &[&str], label_map: &std::collections::HashMap<String, usize>) -> Option<Instruction> {
    match parts {
        ["MOV", reg, src] => Some(Instruction::MOV(parse_reg(reg), parse_source(src))),
        ["PRINT", reg] => Some(Instruction::PRINT(parse_reg(reg), true)),
        ["PRINT", reg, opt] if *opt == "-N" => Some(Instruction::PRINT(parse_reg(reg), false)),
        ["PRINTCH", reg] => Some(Instruction::PRINTCH(parse_reg(reg), true)),
        ["PRINTCH", reg, opt] if *opt == "-N" => Some(Instruction::PRINTCH(parse_reg(reg), false)),
        ["ADD", reg, src] => Some(Instruction::ADD(parse_reg(reg), parse_source(src))),
        ["SUB", reg, src] => Some(Instruction::SUB(parse_reg(reg), parse_source(src))),
        ["MUL", reg, src] => Some(Instruction::MUL(parse_reg(reg), parse_source(src))),
        ["MULH", dest, src1, src2] => Some(Instruction::MULH(parse_reg(dest), parse_reg(src1), parse_reg(src2))),
        ["DIV", reg, src] => Some(Instruction::DIV(parse_reg(reg), parse_source(src))),
        ["MOD", reg, src] => Some(Instruction::MOD(parse_reg(reg), parse_source(src))),
        ["STORE", reg, src] => Some(Instruction::STORE(parse_reg(reg), parse_mem_src(src))),
        ["JMP", addr] => Some(Instruction::JMP(parse_label_or_addr(addr, label_map))),
        ["JZ", addr] => Some(Instruction::JZ(parse_label_or_addr(addr, label_map))),
        ["JNZ", addr] => Some(Instruction::JNZ(parse_label_or_addr(addr, label_map))),
        ["LOOP", addr, reg] => Some(Instruction::LOOP(parse_label_or_addr(addr, label_map), parse_reg(reg))),
        ["INPUT", reg] => Some(Instruction::INPUT(parse_reg(reg))),
        ["DRAW", x, y, src] => Some(Instruction::DRAW(parse_source(x), parse_source(y), parse_source(src))),
        ["SLP", duration] => Some(Instruction::SLP(duration.parse().unwrap())),
        ["CMP", reg, src] => Some(Instruction::CMP(parse_reg(reg), parse_source(src))),
        ["RENDER"] => Some(Instruction::RENDER),
        ["CLS"] => Some(Instruction::CLS),
        ["CTS"] => Some(Instruction::CTS),
        ["HALT"] => Some(Instruction::HALT),
        _ => None,
    }
}

pub fn parse_program(file_path: Option<&str>) -> (Vec<Instruction>, bool) {
    let mut debug_mode = false;
    if let Some(path) = file_path {
        let content = std::fs::read_to_string(path).expect("Failed to read file");
        let mut label_map = std::collections::HashMap::new();
        let mut instructions_raw = Vec::new();
        let mut pc = 0;
        for line in content.lines() {
            let line = line.trim();
            let line = if let Some(comment_start) = line.find("//") {
                &line[..comment_start].trim()
            } else {
                line
            };
            for segment in line.split(';') {
                let segment = segment.trim();
                if segment.is_empty() {
                    continue;
                }
                if segment.ends_with(':') {
                    let label = segment.trim_end_matches(':').to_string();
                    label_map.insert(label, pc);
                } else {
                    instructions_raw.push(segment.to_string());
                    pc += 1;
                }
            }
        }
        let instructions = instructions_raw.iter().map(|segment| {
            let parts: Vec<&str> = segment.split_whitespace().collect();
            parse_instruction(&parts, &label_map)
                .unwrap_or_else(|| panic!("Unknown instruction: {}", segment))
        }).collect();
        (instructions, debug_mode)
    } else {
        center_print("IDLE MODE", 80);
        println!("No file provided. Enter instructions manually:");
        println!("{}", "-".repeat(82));
        println!("Type 'RUN' to stop the program.");
        println!("{}", "-".repeat(82));
        let mut label_map = std::collections::HashMap::new();
        let mut instructions_raw = Vec::new();
        let mut pc = 0;
        loop {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let line = input.trim();
            if line.eq_ignore_ascii_case("RUN") {
                break;
            }

            let line = if let Some(comment_start) = line.find("//") {
                &line[..comment_start].trim()
            } else {
                line
            };

            for segment in line.split(';') {
                let segment = segment.trim();
                if segment.is_empty() {
                    continue;
                }
                if segment.ends_with(':') {
                    let label = segment.trim_end_matches(':').to_string();
                    label_map.insert(label, pc);
                } else {
                    instructions_raw.push(segment.to_string());
                    pc += 1;
                }
            }
        }
        let mut program = Vec::new();
        for segment in instructions_raw {
            let parts: Vec<&str> = segment.split_whitespace().collect();
            if let Some(instruction) = parse_instruction(&parts, &label_map) {
                program.push(instruction);
            } else {
                println!("Unknown instruction: {}", segment);
            }
        }
        println!("{}", "-".repeat(82));
        print!("Enable debug mode? (y/n): ");
        let _ = stdout().flush();
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        debug_mode = input.trim().eq_ignore_ascii_case("y");
        (program, debug_mode)
    }
}

fn parse_reg(reg: &str) -> Reg {
    match reg {
        "A" => Reg::A,
        "B" => Reg::B,
        "C" => Reg::C,
        "D" => Reg::D,
        "E" => Reg::E,
        _ => panic!("Unknown register: {}", reg),
    }
}

fn parse_source(src: &str) -> Source {
    if let Ok(lit) = src.parse::<u8>() {
        Source::Lit(lit)
    } else if src.starts_with("'") && src.ends_with("'") && src.len() == 3 {
        let char_value = src.chars().nth(1).unwrap() as u8;
        Source::Lit(char_value)
        // chars
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
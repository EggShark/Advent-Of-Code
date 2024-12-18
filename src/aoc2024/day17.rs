use crate::{Solve, DOUBLE_NEW_LINE};
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut registers = [0; 3];
    let mut out: Vec<u8> = vec![];

    let (reg_values, instructions) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();
    let instructions = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    for (idx, line) in reg_values.lines().enumerate() {
        let (_, right) = line.split_once(": ").unwrap();
        registers[idx] = right.parse::<u64>().unwrap();
    }

    run(registers, &instructions, &mut out);

    let mut formated = String::new();
    for value in out {
        formated.push_str(&value.to_string());
        formated.push(',');
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(formated),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let (_, instructions) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();
    let instructions = instructions
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();


    let solve = p2(&instructions, instructions.len() - 1, 0).unwrap();



    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn run(mut registers: [u64; 3], program: &[u8], out: &mut Vec<u8>) {
    let mut pc = 0;
    while pc < program.len()-1 {
        let instruction = Instruction::from(program[pc]);
        let mut jumped = false;
        match instruction {
            Instruction::DivideA => {
                let numerator = registers[0];
                let denom = get_combo(program[pc+1], &registers);
                registers[0] = numerator/(2_u64.pow(denom as u32));
            }
            Instruction::Xor => {
                registers[1] ^= program[pc+1] as u64;
            }
            Instruction::Mod8 => {
                let value = get_combo(program[pc+1], &registers);
                registers[1] = value % 8;
            }
            Instruction::Jump => {
                if registers[0] != 0 {
                    pc = program[pc+1] as usize;
                    jumped = true;
                }
            }
            Instruction::RegXor => {
                registers[1] ^= registers[2];
            }
            Instruction::Out => {
                out.push((get_combo(program[pc+1], &registers) % 8) as u8);
            }
            Instruction::DivideB => {
                let numerator = registers[0];
                let denom = get_combo(program[pc+1], &registers);
                registers[1] = numerator/(2_u64.pow(denom as u32));
            }
            Instruction::DivideC => {
                let numerator = registers[0];
                let denom = get_combo(program[pc+1], &registers);
                registers[2] = numerator/(2_u64.pow(denom as u32));
            }
        }
        if !jumped {
            pc += 2;
        }
    }
}

fn p2(code: &[u8], i: usize, mut new_a: u64) -> Option<u64> {
    let target = &code[i..];
    new_a *= 8;
    for n in new_a..new_a+8 {
        let mut out = Vec::new();
        run([n, 0, 0], code, &mut out);
        if out.ends_with(target) {
            if i == 0 {
                return Some(n);
            }
            if let Some(n) = p2(code, i - 1, n) {
                return Some(n)
            }
        }
    }
    None
}

fn get_combo(literal: u8, registers: &[u64; 3]) -> u64 {
    match literal {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        7 | _ => unreachable!(),
    }
}

enum Instruction {
    DivideA, // 0
    Xor, // 1
    Mod8, // 2
    Jump, // 3
    RegXor, // 4
    Out, // 5
    DivideC, // 6
    DivideB, // 7
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::DivideA,
            1 => Self::Xor,
            2 => Self::Mod8,
            3 => Self::Jump,
            4 => Self::RegXor,
            5 => Self::Out,
            6 => Self::DivideB,
            7 => Self::DivideC,
            _ => unreachable!("Not a valid program")
        }
    }
}
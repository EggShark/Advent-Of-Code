use crate::{Solve, DOUBLE_NEW_LINE};
use std::{collections::HashMap, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let (default_values, instructions) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut wires = default_values
        .lines()
        .map(|l| {
            let (name, value) = l.split_once(": ").unwrap();
            let value = value.parse::<u64>().unwrap();
            (name, value)
        })
        .collect::<HashMap<&str, u64>>();

    let instructions = instructions
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            let left = words.next().unwrap();
            let op = match words.next().unwrap() {
                "XOR" => Op::Xor,
                "AND" => Op::And,
                "OR" => Op::Or,
                _ => unreachable!("erm")
            };
            let right = words.next().unwrap();
            assert_eq!(words.next(), Some("->"));
            let storer = words.next().unwrap();
            let int = Instruction::new(left, right, op);
            (storer, int)
        })
        .collect::<HashMap<_, _>>();

    let mut keys_to_search = instructions.keys().copied().filter(|k| k.starts_with("z")).collect::<Vec<_>>();
    keys_to_search.sort_unstable_by(|l, r| r.cmp(l));

    let solve = keys_to_search
        .into_iter()
        .map(|key| calculate_wire(key, &mut wires, &instructions))
        .fold(0, |acc, x| (acc << 1) + x);

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let (_, instructions) = data_in.split_once(DOUBLE_NEW_LINE).unwrap();

    let mut instructions = instructions
        .lines()
        .map(|line| {
            let mut words = line.split(' ');
            let left = words.next().unwrap();
            let op = match words.next().unwrap() {
                "XOR" => Op::Xor,
                "AND" => Op::And,
                "OR" => Op::Or,
                _ => unreachable!("erm")
            };
            let right = words.next().unwrap();
            assert_eq!(words.next(), Some("->"));
            let storer = words.next().unwrap();
            let int = Instruction::new(left, right, op);
            (storer, int)
        })
        .collect::<HashMap<_, _>>();

    let keys = instructions.keys().copied().collect::<Vec<_>>();
    let mut swapped = vec![];
    
    for _ in 0..4 {
        let base = depth(&instructions);
        'o: for &x in keys.iter() {
            for &y in keys.iter() {
                if x == y {
                    continue;
                }

                let left = instructions.get(x).map(|&u| u.clone()).unwrap();
                let right = instructions.get(y).map(|&u| u.clone()).unwrap();

                instructions.insert(x, right.clone());
                instructions.insert(y, left.clone());
                
                if depth(&instructions) > base {
                    swapped.extend_from_slice(&[x, y]);
                    break 'o;
                }

                instructions.insert(x, left);
                instructions.insert(y, right);
            }
        }
    }

    swapped.sort();
    let solve = swapped.join(","); 

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn calculate_wire<'a>(wire: &'a str, known: &mut HashMap<&'a str, u64>, instructions: &HashMap<&'a str, Instruction<'a>>) -> u64 {
    if let Some(&v) = known.get(wire) {
        return v;
    }

    let instruction = instructions.get(wire).unwrap();
    let value = match instruction.operation {
        Op::And => calculate_wire(&instruction.left, known, instructions) & calculate_wire(&instruction.right, known, instructions),
        Op::Or => calculate_wire(&instruction.left, known, instructions) | calculate_wire(&instruction.right, known, instructions),
        Op::Xor => calculate_wire(&instruction.left, known, instructions) ^ calculate_wire(&instruction.right, known, instructions),
    };
    known.insert(wire, value);

    value
}

fn verifiy_z<'a>(wire: &'a str, num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    if !instructions.contains_key(wire) {
        return false;
    }

    let inst = instructions.get(wire).unwrap();
    if inst.operation != Op::Xor {
        return false;
    }
    
    if num == 0 {
        let mut test = [inst.left, inst.right];
        test.sort();
        return test == ["x00", "y00"];
    }
    
    (verifiy_i_xor(&inst.left, num, instructions) && verifiy_carry(&inst.right, num, instructions))
        || (verifiy_i_xor(&inst.right, num, instructions) && verifiy_carry(&inst.left, num, instructions))
}

fn verifiy_i_xor<'a>(wire: &'a str, num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    if !instructions.contains_key(wire) {
        return false;
    }

    let inst = instructions.get(wire).unwrap();
    if inst.operation != Op::Xor {
        return false;
    }

    let mut test = [inst.left, inst.right];
    test.sort();

    test == [&make_wire("x", num), &make_wire("y", num)]
}

fn verifiy_carry<'a>(wire: &'a str, num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    if !instructions.contains_key(wire) {
        return false;
    }

    let inst = instructions.get(wire).unwrap();
    if num == 1 {
        if inst.operation != Op::And {
            return false;
        }

        let mut test = [inst.left, inst.right];
        test.sort();
        return test == ["x00", "y00"];
    }
    if inst.operation != Op::Or {
        return  false;
    }
    
    verifiy_d_carry(&inst.left, num - 1, instructions) && verifiy_recarry(&inst.right, num - 1, instructions)
        || verifiy_d_carry(&inst.right, num - 1, instructions) && verifiy_recarry(&inst.left, num - 1, instructions)
}

fn verifiy_recarry<'a>(wire: &'a str, num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    if !instructions.contains_key(wire) {
        return false;
    }

    let inst = instructions.get(wire).unwrap();
    if inst.operation != Op::And {
        return false
    }

    verifiy_i_xor(&inst.left, num, instructions) && verifiy_carry(&inst.right, num, instructions)
        || verifiy_i_xor(&inst.right, num, instructions) && verifiy_carry(&inst.left, num, instructions)
}

fn verifiy_d_carry<'a>(wire: &'a str, num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    if !instructions.contains_key(wire) {
        return false;
    }

    let inst = instructions.get(wire).unwrap();
    if inst.operation != Op::And {
        return false;
    }
    let mut test= [inst.left, inst.right];
    test.sort();
    test == [&make_wire("x", num), &make_wire("y", num)]
}

fn make_wire(letter: &str, num: u64) -> String {
    format!("{}{:02}", letter, num)
}

fn verify<'a>(num: u64, instructions: &HashMap<&'a str, Instruction<'a>>) -> bool {
    verifiy_z(&make_wire("z", num), num, &instructions)
}

fn depth<'a>(instructions: &HashMap<&'a str, Instruction<'a>>) -> u64 {
    let mut i = 0;
    while verify(i, instructions) {
        i += 1;
    }

    i
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    left: &'a str,
    right: &'a str,
    operation: Op,
}

impl<'a> Instruction<'a> {
    fn new(left: &'a str, right: &'a str, operation: Op) -> Self {
        Self {
            left,
            right,
            operation,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Or,
    Xor,
    And,
}
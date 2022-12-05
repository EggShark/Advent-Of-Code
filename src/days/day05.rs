use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day05.txt").unwrap();
    let time = Instant::now();

    let instructions = text.split("\n\n").collect::<Vec<&str>>();
    
    let stack = instructions[0];

    let mut pain: Vec<Vec<String>> = vec![Vec::new(); 9];
    for line in stack.lines() {
        let mut counter = 0;
        let mut pos = 0;
        let mut iter = line.chars();
        let _skip = iter.next().unwrap(); // used to skip first bracket
        for char in iter {
            if char.is_alphabetic() {
                pain[pos].push(char.to_string());
            } 
            if counter % 4 == 0 {
                pos += 1;
            }
            counter += 1;

        }
    }
    let mut stacks1: Vec<Stack> = Vec::new();
    let mut stacks2: Vec<Stack> = Vec::new();

        
    for stack in pain.iter_mut() {
        stacks1.push(Stack{stack: stack.to_vec()});
    }

    for mut stack in pain {
        stack.reverse();
        stacks2.push(Stack{stack});
    }

    let instructions = instructions[1];
    let instructions: Vec<[usize; 3]> = instructions.lines().map(|line| {
        let x: Vec<&str> = line.split_whitespace().collect();
        println!("{:?}", x);
        let a = x[1].parse::<usize>().unwrap();
        let b = x[3].parse::<usize>().unwrap();
        let c = x[5].parse::<usize>().unwrap();
        [a, b, c]
    }).collect(); 

    // part 1 solve
    for instruction in instructions.iter() {
        let amount = instruction[0];
        for _ in 0..amount {
            let x = stacks1[instruction[1] - 1].stack.pop().unwrap();
            stacks1[instruction[2] - 1].stack.push(x);
        }
    }

    // part 2
    for instruction in instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let mut a = Vec::new();
        for i in stacks2[from].stack.len() - amount..stacks2[from].stack.len() {
            a.push(stacks2[from].stack[i].to_owned());
        }
        for _ in 0..amount {
            stacks2[from].stack.pop();
        }
        for thing in a {
            stacks2[to].stack.push(thing);
        }
    }

    let mut sol1 = String::new();
    let mut sol2 = String::new();

    for stack in stacks1 {
        sol1 += &stack.stack[stack.stack.len() - 1];
    }
    for stack in stacks2 {
        sol2 += &stack.stack[stack.stack.len() - 1];
    }

    let solution = (SolutionType::Str(sol1), SolutionType::Str(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug)]
struct Stack {
    stack: Vec<String>,
}

// part 1 solve
// for instruction in instructions {
//     let amount = instruction[0];
//     for _ in 0..amount {
//         let x = stacks[instruction[1] - 1].stack.pop().unwrap();
//         stacks[instruction[2] - 1].stack.push(x);
//     }
// }
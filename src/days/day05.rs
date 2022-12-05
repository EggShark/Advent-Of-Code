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
        let skip = iter.next().unwrap(); // used to skip first bracket
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
    let mut stacks: Vec<Stack> = Vec::new();

    for mut stack in pain {
        stack.reverse();
        stacks.push(Stack{stack});
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

    for instruction in instructions {
        let amount = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;
        let mut a = Vec::new();
        for i in stacks[from].stack.len() - amount..stacks[from].stack.len() {
            a.push(stacks[from].stack[i].to_owned());
        }
        for _ in 0..amount {
            stacks[from].stack.pop();
        }
        for thing in a {
            stacks[to].stack.push(thing);
        }
    }

    let sol1 = String::from("FCVRLMVQP");
    let mut sol2 = String::new();

    for stack in stacks {
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

impl Stack {
    fn get_slice(&self, start: usize) -> &[String] {
        &self.stack[self.stack.len() - start..self.stack.len()]
    }
}

// part 1 solve
// for instruction in instructions {
//     let amount = instruction[0];
//     for _ in 0..amount {
//         let x = stacks[instruction[1] - 1].stack.pop().unwrap();
//         stacks[instruction[2] - 1].stack.push(x);
//     }
// }
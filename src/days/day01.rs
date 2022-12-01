
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...

    let input = read_to_string("./texts/day01.txt").unwrap();
    let tool: Vec<&str> = input.split("\n").collect();

    println!("{:?}", tool);

    let mut largest_sum = 0;
    let mut current_sum = 0;

    let mut sum2 = 0;
    let mut sum3 = 0;

    for number in tool {
        if number == "" {
            current_sum = 0;
        } else {
            let u: u64 = number.parse().unwrap();
            current_sum += u;
            if current_sum > largest_sum {
                largest_sum = current_sum;
            } else if current_sum > sum2 {
                sum2 = current_sum;
            } else if current_sum > sum3 {
                sum3 = current_sum;
            }
        }

    }

    let sol1: u64 = largest_sum;
    let sol2: u64 = largest_sum + sum2 + sum3;

    (Solution::U64(sol1), Solution::U64(sol2))
}
use crate::{Solution, SolutionType};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day25").unwrap();
    let time = Instant::now();

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    text.lines()
        .for_each(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let parent = graph.entry(left).or_insert(HashSet::new());
            let right1 = right.split_whitespace();
            for child in right1 {
                parent.insert(child);
            }

            let right = right.split_whitespace();
            for child in right {
                let c = graph.entry(child).or_insert(HashSet::new());
                c.insert(left);
            }
        });

    // finds edges with the most connections
    let mut frequency = HashMap::new();
    for start in graph.keys() {
        let mut queue = VecDeque::new();
        queue.push_back(start);
        let mut seen = HashSet::new();
        seen.insert(start);

        while let Some(pos) =  queue.pop_front() {
            for next in graph[pos].iter() {
                if seen.insert(next) {
                    let key = if pos < next {
                        [pos, next]
                    } else {
                        [next, pos]
                    };

                    let e = frequency.entry(key).or_insert(0);
                    *e += 1;

                    queue.push_back(next);
                }
            }
        }
    }

    // sorts by frequency
    let mut order: Vec<([&&str; 2], i32)> = frequency.into_iter().collect();
    order.sort_unstable_by_key(|e| e.1);
    order.reverse();

    let cut: Vec<[&&str; 2]> = order.iter().take(3).map(|p| p.0).collect();
    let start = *graph.keys().next().unwrap();
    let mut size = 1;

    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some(ref pos) = queue.pop_front() {
        for next in graph[pos].iter() {
            let key = if pos < next {
                [pos, next]
            } else {
                [next, pos]
            };

            if cut.contains(&key) {
                continue;
            }

            if seen.insert(next) {
                size += 1;
                queue.push_back(next);
            }
        }
    }

    let sol1: u64 = size * (graph.len() as u64 - size);
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}
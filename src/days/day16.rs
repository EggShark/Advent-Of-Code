use crate::{Solution, SolutionType};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day16.txt").unwrap();
    let time = Instant::now();

    let sol1: i32 = part_1(TESTINPUT);
    let sol2: u64 = 0;

    let solution = (SolutionType::I32(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn part_1(input: &str) -> i32 {
    let mut valves = HashMap::new();
    let mut tunnels = HashMap::new();
    for line in input.lines() {
        let valve = &line[6..8];
        let (left, right) = line.split_once("; ").unwrap();
        let (_, num) = left.split_once("=").unwrap();
        let flow = num.parse::<i8>().unwrap();
        let leads_to = if let Some(singular) = right.strip_prefix("tunnel leads to valve ") {
            vec![singular]
        } else if let Some(many) = right.strip_prefix("tunnels lead to valves ") {
            many.split(", ").collect()
        } else {
            vec![]
        };
        valves.insert(valve, flow);
        tunnels.insert(valve, leads_to);
    }

    let mut dists = HashMap::new();
    for (name, flow) in &valves {
        if *name != "AA" && flow == &0 {
            continue;
        }
        let mut visted = HashSet::new();
        let mut que = VecDeque::new();
        visted.insert(*name);
        que.push_front((0, *name));

        let mut temp = HashMap::new();
        temp.insert("AA", 0);
        dists.insert(*name, temp);

        while let Some((distance, position)) = que.pop_back() {
            for neigbor in tunnels.get(position).unwrap() {
                if visted.insert(neigbor) {
                    if valves[neigbor] == 0 {
                        let z = dists.get_mut(name).unwrap().get_mut(neigbor);
                        match z {
                            Some(i) => {*i = distance + 1;},
                            None => {dists.get_mut(name).unwrap().insert(neigbor, distance + 1);},
                        }
                    }
                    que.push_front((distance + 1, neigbor));
                } else {
                    continue;
                }
            }
        }
        dists.get_mut(name).unwrap().remove(name);
        if *name != "AA" {
            dists.get_mut(name).unwrap().remove("AA");
        }
    }
    println!("{:?}", dists);
    0
}
const TESTINPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part_1(TESTINPUT), 1651)
    }
}
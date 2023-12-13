use crate::{Solution, SolutionType};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day12").unwrap();
    let time = Instant::now();

//     let text = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";

    let mut cache: HashMap<(Vec<char>, Vec<usize>), u64> = HashMap::new();

    let p1: u64 = text
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(left, right)| {
            count(
                &left.chars().collect::<Vec<char>>(),
                &right.split(",").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
                &mut cache,
            )
        }).sum();

    cache.clear();
    let p2: u64 = text
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(left, right)| {
            let cfg = left
                .chars()
                .chain(['?'])
                .cycle()
                .take(left.len() * 5 + 4)
                .collect::<Vec<char>>();
            let nums = right
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let len = nums.len();
            let nums = nums
                .into_iter()
                .cycle()
                .take(len * 5)
                .collect::<Vec<usize>>();

            count(&cfg, &nums, &mut cache)
        }).sum();

    let sol1: u64 = p1;
    let sol2: u64 = p2;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn count(cfg: &[char], nums: &[usize], cahce: &mut HashMap<(Vec<char>, Vec<usize>), u64>) -> u64 {
    if cfg.is_empty() {
        if nums.is_empty() {
            // no numbers and no space there is 1 valid way to handle that
            return 1;
        } else {
            // more blocks left and yet no more room so it is invalid
            return 0;
        }
    }

    if nums.is_empty() {
        if cfg.contains(&'#') {
            // expects 0 spring blocks yet found one so not a valid config
            return 0;
        } else {
            // no numbers no blocks its valid in only 1 way
            return 1;
        }
    }

    // checks if we've already been here before (need to find away to remove heap alloc)
    // and if we have just return that
    let key = (cfg.to_vec(), nums.to_vec());
    if let Some(val) = cahce.get(&key) {
        return *val;
    }

    let mut result = 0;

    // if the first cfg is broken or ? we just see what its like if we 
    // treat it as broken
    if ".?".contains(cfg[0]) {
        result += count(&cfg[1..], nums, cahce)
    }

    // #? marks the start of the block
    if "#?".contains(cfg[0]) {
        // nums[0] <= cfg.len() checks to see if there are enough springs left
        // !cfg[..nums[0]] checks to see if all the springs before num are broken
        // (nums[0] == cfg.len() || cfg[nums[0]] != '#') checks again to see if its one large block at the end
        // or if there are more springs afterwards it must not be broken
        if nums[0] <= cfg.len() && !cfg[..nums[0]].contains(&'.') && (nums[0] == cfg.len() || cfg[nums[0]] != '#') {
            // starts a new block at cfg[0] that is the lenging of nums[0] + bounds check
            // we add + 1 to make sure that it does not start a new block imeadiatly
            if !(nums[0]+1 > cfg.len() - 1) {
                result += count(&cfg[nums[0]+1..], &nums[1..], cahce)
            } else {
                result += count(&[], &nums[1..], cahce)
            }
        }
    }

    cahce.insert((cfg.to_vec(), nums.to_vec()), result);

    result
}
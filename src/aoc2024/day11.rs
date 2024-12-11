use crate::Solve;
use std::{collections::HashMap, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut stones = data_in.split(' ').map(|d| d.parse().unwrap()).collect::<Vec<u64>>();

    for _ in 0..25 {
        let mut i = 0;
        loop {
            if i >= stones.len() {
                break;
            }

            if stones[i] == 0 {
                stones[i] = 1;
            } else if (stones[i].ilog10() + 1) % 2 == 0 {
                let num_str = stones[i].to_string();
                let left = &num_str[..num_str.len()/2];
                let right = &num_str[num_str.len()/2..];
                stones[i] = left.parse().unwrap();
                stones.insert(i+1, right.parse().unwrap());
                i += 1;
            } else {
                stones[i] *= 2024;
            }



            i += 1;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(stones.len()),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = 0;

    let stones = data_in.split(' ').map(|d| d.parse().unwrap()).collect::<Vec<u64>>();

    let mut sum = 0;
    let mut map: HashMap<(u64, usize), u64> = HashMap::new();
    for stone in stones {
        sum += count_splits(stone, 75, &mut map);
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(sum),
        time_ms,
    }
}

fn count_splits(num: u64, times: usize, map: &mut HashMap<(u64, usize), u64>) -> u64 {
    if times == 0 {
        return 1;
    }

    if let Some(h) = map.get(&(num, times)) {
        return *h;
    } 

    let steps = if num == 0 {
        count_splits(1, times - 1, map)
    } else if (num.ilog10() + 1) % 2 == 0 {
        let num_str = num.to_string();
        let left = num_str[..num_str.len()/2].parse::<u64>().unwrap();
        let right = num_str[num_str.len()/2..].parse::<u64>().unwrap();
        count_splits(left, times - 1, map) + count_splits(right, times - 1, map)
    } else {
        count_splits(num * 2024, times - 1, map)
    };
    map.insert((num, times), steps);

    steps
}
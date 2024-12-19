use crate::Solve;
use std::{cmp::min, collections::{HashMap, HashSet}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0;

    let mut lines = data_in.lines();
    let patterns = lines.next().unwrap().split(", ");
    let max_len = patterns.clone().map(|d| d.len()).max().unwrap();
    let patterns = patterns.collect::<HashSet<_>>();
    let mut cache = HashMap::new();

    
    for design in lines.skip(1) {
        if good_towel(design, &patterns, max_len, &mut cache) {
            solve += 1;
        }
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve: u64 = 0;

    let mut lines = data_in.lines();
    let patterns = lines.next().unwrap().split(", ");
    let max_len = patterns.clone().map(|d| d.len()).max().unwrap();
    let patterns = patterns.collect::<HashSet<_>>();
    let mut cache = HashMap::new();
    
    
    for design in lines.skip(1) {
        solve += count_towel(design, &patterns, max_len, &mut cache);
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn good_towel<'a>(towel: &'a str, patterns: &HashSet<&str>, max_len: usize, cache: &mut HashMap<&'a str, bool>) -> bool {
    if towel.len() == 0 {
        return true;
    }

    if let Some(n) = cache.get(towel) {
        return *n;
    }

    let loop_amount = min(towel.len(), max_len) + 1;
    for i in 0..loop_amount {
        if patterns.contains(&towel[..i]) && good_towel(&towel[i..], patterns, max_len, cache) {
            cache.insert(towel, true);
            return true;
        }
    }

    cache.insert(towel, false);
    false
}

fn count_towel<'a>(towel: &'a str, patterns: &HashSet<&str>, max_len: usize, cache: &mut HashMap<&'a str, u64>) -> u64 {
    if towel.len() == 0 {
        return 1;
    }

    if let Some(n) = cache.get(towel) {
        return *n;
    }

    let mut counter = 0;
    let loop_amount = min(towel.len(), max_len) + 1;
    for i in 0..loop_amount {
        if patterns.contains(&towel[..i]) {
            counter += count_towel(&towel[i..], patterns, max_len, cache);
        }
    }

    cache.insert(towel, counter);
    counter
}
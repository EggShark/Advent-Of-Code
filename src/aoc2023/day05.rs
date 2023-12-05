use crate::{Solution, SolutionType};
use std::cmp::min;
use std::ops::Range;
use std::fs::read_to_string;
use std::str::Lines;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day6").unwrap();
    let time = Instant::now();

    let mut lines = text.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let _ = lines.next().unwrap();
    let _ = lines.next().unwrap();

    let seed_to_soil = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let soil_to_fert = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let fert_to_water = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let water_to_light = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let light_to_temp = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let temp_to_humidity = make_value(&mut lines);
    let _ = lines.next().unwrap();
    let humidity_to_location = make_value(&mut lines);

    let p1 = seeds
        .iter()
        .map(|seed| update_value(*seed, &seed_to_soil))
        .map(|seed| update_value(seed, &soil_to_fert))
        .map(|seed| update_value(seed, &fert_to_water))
        .map(|seed| update_value(seed, &water_to_light))
        .map(|seed| update_value(seed, &light_to_temp))
        .map(|seed| update_value(seed, &temp_to_humidity))
        .map(|seed| update_value(seed, &humidity_to_location))
        .min()
        .unwrap();

    let mut p2_seeds = Vec::new();

    let mut maps = vec![
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    ];

    for idx in (0..seeds.len()).step_by(2) {
        let start = seeds[idx];
        let to_add = seeds[idx + 1];
        let end = start + to_add;
        p2_seeds.push(start..end)
    }

    println!("{:?}", maps[6]);

    let sol1: u64 = p1;
    let sol2: u64 = 0;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

// fn move_range(seed_range: &Range<u64>, map_ranges: &[(Range<u64>, Range<u64>)]) -> Vec<Range<u64>> {
//     let mut out = Vec::new();
//     let start = seed_range.start;
//     let mut current = start;

//     for (dst, src) in map_ranges.iter().skip_while(|(_, src)| src.end < start) {
//         if src.start > current {
//             out.push(current..min(seed_range.end, src.start));
//             current = src.start;
//         }
//         if current >= seed_range.end {
//             break;
//         }
//         out.push(shift(current, dst, src)..shift(min(seed_range.end, src.end), dst, src));
//         current = src.end;
//         if current >= seed_range.end {
//             break;
//         }
//     }
//     if current < seed_range.end {
//         out.push(current..seed_range.end);
//     }

//     out
// }

fn make_value(lines: &mut Lines<'_>) -> Vec<(Range<u64>, Range<u64>)> {
    let mut vec = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut numbers = line
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|f| f.parse::<u64>().unwrap());
        let dst = numbers.next().unwrap();
        let source = numbers.next().unwrap();
        let range = numbers.next().unwrap();
        vec.push((dst..range+dst, source..source+range));
    }
    vec
}

fn update_value(val: u64, ranges: &Vec<(Range<u64>, Range<u64>)>) -> u64 {
    let found = ranges
        .iter()
        .find(|(_, src)| val >= src.start && val <= src.end);

    match found {
        Some(ranges) => shift(val,&ranges.0,&ranges.1),
        None => val
    }
}

fn shift(num: u64, r1: &Range<u64>, r2: &Range<u64>) -> u64 {
    if r2.start > r1.start {
        num - (r2.start - r1.start)
    } else {
        num + (r1.start - r2.start)
    }
}
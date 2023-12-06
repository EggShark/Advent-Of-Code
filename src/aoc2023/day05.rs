use crate::{Solution, SolutionType};
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
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

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

    maps.iter_mut().for_each(|map| map.sort_unstable_by_key(|f| f.end));

    for idx in (0..seeds.len()).step_by(2) {
        let start = seeds[idx];
        let to_add = seeds[idx + 1];
        let end = start + to_add;
        p2_seeds.push(Range::new(start, end))
    }

    p2_seeds.sort_unstable_by_key(|f| f.start);

    println!("len: {}", p2_seeds.len());
    join_ranges(&mut p2_seeds);
    println!("len: {}", p2_seeds.len());

    for map in maps {
        let mut next_ranges = Vec::new();
        let mut range_iter = p2_seeds.iter_mut();
        let mut translation_iter = map.iter();

        let mut range = range_iter.next().unwrap();
        let mut translation = translation_iter.next().unwrap();
        
        loop {
            if range.start > translation.end {
                if let Some(next) = translation_iter.next() {
                    translation = next;
                    continue;
                } else {
                    next_ranges.push(*range);
                    for range in range_iter {
                        next_ranges.push(*range);
                    }
                    break;
                }
            } else {
                if range.start < translation.start && range.end > translation.start {
                    next_ranges.push(Range::new(range.start, translation.start - 1));
                    range.start = translation.start;
                }

                if range.start >= translation.start {
                    if range.end > translation.end {
                        next_ranges.push(Range::new(
                            range.start + translation.offset,
                            translation.end + translation.offset
                        ));
                        range.start = translation.end + 1;
                    } else {
                        next_ranges.push(Range::new(
                            range.start + translation.offset,
                            range.end + translation.offset,
                        ));

                        if let Some(next) = range_iter.next() {
                            range = next;
                        } else {
                            break;
                        }
                    }
                } else {
                    next_ranges.push(*range);
                    if let Some(next) = range_iter.next() {
                        range = next;
                    } else {
                        break;
                    }
                }
            }
        }

        p2_seeds = next_ranges;
        p2_seeds.sort_unstable_by_key(|range| range.start);
        join_ranges(&mut p2_seeds);
    }

    println!("p2 seeds: {:?}", p2_seeds);

    let sol1: i64 = p1;
    let sol2: i64 = p2_seeds.first().unwrap().start - 1;

    let solution = (SolutionType::I64(sol1), SolutionType::I64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct MapRange {
    start: i64,
    end: i64,
    offset: i64,
}

impl MapRange {
    pub fn new(start: i64, end: i64, offset: i64) -> Self {
        Self {
            start,
            end,
            offset,
        }
    }
}

fn join_ranges(ranges: &mut Vec<Range>) {
    let mut offset = 0;
    let len = ranges.len() - 1;

    for i in 0..len {
        if offset + i > len {
            break;
        }
        let next = ranges[i + 1 - offset];
        let current = ranges.get_mut(i).unwrap();

        if current.end >= next.start {
            current.end = next.end;
            ranges.remove(i + 1 - offset);
            offset += 1;
        }
    }
}

fn make_value(lines: &mut Lines<'_>) -> Vec<MapRange> {
    let mut vec = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut numbers = line
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|f| f.parse::<i64>().unwrap());
        let dst = numbers.next().unwrap();
        let source = numbers.next().unwrap();
        let range = numbers.next().unwrap();
        vec.push(MapRange::new(source, source + range, dst-source));
    }
    vec
}

fn update_value(val: i64, ranges: &Vec<MapRange>) -> i64 {
    let found = ranges
        .iter()
        .find(|range| val >= range.start && val <= range.end);

    match found {
        Some(range) => val + range.offset,
        None => val
    }
}
use crate::Solve;
use std::{collections::{HashMap, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0_usize;
    
    let mut cache = HashMap::new();
    for line in data_in.lines() {
        let num = line[0..3].parse::<usize>().unwrap();
        let mut current_pos = (3, 2);
        let mut paths = vec![];
        for b in line.bytes() {
            let next_pos = b_to_num_pad(b);
            paths.push(path(current_pos, next_pos, true));
            current_pos = next_pos;
        }

        let len = paths.into_iter().map(|p| dfs(&mut cache, (2, p))).sum::<usize>();
        solve += len * num;
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0_usize;
    
    let mut cache = HashMap::new();
    for line in data_in.lines() {
        let num = line[0..3].parse::<usize>().unwrap();
        let mut current_pos = (3, 2);
        let mut paths = vec![];
        for b in line.bytes() {
            let next_pos = b_to_num_pad(b);
            paths.push(path(current_pos, next_pos, true));
            current_pos = next_pos;
        }

        let len = paths.into_iter().map(|p| dfs(&mut cache, (25, p))).sum::<usize>();
        solve += len * num;
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn path(from: (u8, u8), to: (u8, u8), numpad: bool) -> Vec<u8> {
    let horiz_iter =
        std::iter::repeat_n(if to.1 > from.1 { b'>' } else { b'<' }, to.1.abs_diff(from.1) as usize);
    let vert_iter =
        std::iter::repeat_n(if to.0 > from.0 { b'v' } else { b'^' }, to.0.abs_diff(from.0) as usize);
    let a_iter = std::iter::once(b'A');
    let blocked = if numpad {
        (from.0 == 3 && to.1 == 0) || (from.1 == 0 && to.0 == 3)
    } else {
        (from.1 == 0 && to.0 == 0) || (from.0 == 0 && to.1 == 0)
    };

    if (to.1 < from.1) == blocked {
        vert_iter.chain(horiz_iter).chain(a_iter).collect()
    } else {
        horiz_iter.chain(vert_iter).chain(a_iter).collect()
    }
}

fn dfs(cache: &mut HashMap<(u8, Vec<u8>), usize>, block: (u8, Vec<u8>)) -> usize {
    if let Some(&x) = cache.get(&block) {
        return x;
    }

    if block.0 == 0 {
        return block.1.len();
    }

    // i prefer x y sorry mister
    let mut pos = (0, 2);
    let mut paths = vec![];
    for &b in block.1.iter() {
        let next_pos = b_to_dir(b);
        paths.push(path(pos, next_pos, false));
        pos = next_pos;
    }

    let distance = paths.into_iter().map(|p| dfs(cache, (block.0 - 1, p))).sum::<usize>();

    cache.insert(block, distance);
    distance
}
//      +---+---+
//      | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+
fn b_to_dir(b: u8) -> (u8, u8) {
    match b {
        b'A' => (0, 2),
        b'^' => (0, 1),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => unreachable!(),
    }
}

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
fn b_to_num_pad(b: u8) -> (u8, u8) {
    match b {
        b'1'..=b'9' => {
            let count = b - b'1';
            let column = count % 3;
            let row = 2 - (count / 3);
            (row, column)
        }
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => unreachable!()
    }
}

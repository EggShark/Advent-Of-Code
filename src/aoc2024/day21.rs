use crate::Solve;
use std::{collections::HashMap, time::Instant};
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0_usize;
    
    let mut cache = HashMap::new();
    for line in data_in.lines() {
        let num = line[0..3].parse::<usize>().unwrap();
        let mut current_pos = b_to_num_pad(b'A');
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
        let mut current_pos = b_to_num_pad(b'A');
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

fn path((x1, y1): (u8, u8), (x2, y2): (u8, u8), numpad: bool) -> Vec<u8> {
    let horiz_iter =
        std::iter::repeat_n(if x2 > x1 { b'>' } else { b'<' }, x2.abs_diff(x1) as usize);
    let vert_iter =
        std::iter::repeat_n(if y2 > y1 { b'v' } else { b'^' }, y2.abs_diff(y1) as usize);
    let a_iter = std::iter::once(b'A');
    let blocked = if numpad {
        (y1 == 3 && x2 == 0) || (x1 == 0 && y2 == 3)
    } else {
        (x1 == 0 && y2 == 0) || (y1 == 0 && x2 == 0)
    };

    if (x2 < x1) == blocked {
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
    let mut pos = b_to_dir(b'A');
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
        b'A' => (2, 0),
        b'^' => (1, 0),
        b'<' => (0, 1),
        b'v' => (1, 1),
        b'>' => (2, 1),
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
            (column, row)
        }
        b'0' => (1, 3),
        b'A' => (2, 3),
        _ => unreachable!()
    }
}

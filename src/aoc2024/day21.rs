use crate::Solve;
use std::{collections::{HashMap, VecDeque}, time::Instant};
///////////////////////////////////////////////////////////////////////////////
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+
const NUM_PAD: &[&[u8]] = &[
    &[b'7', b'8', b'9'],
    &[b'4', b'5', b'6'],
    &[b'1', b'2', b'3'],
    &[b'x', b'0', b'A'],
];

const DIR_PAD: &[&[u8]]  = &[
    &[b'x', b'^', b'A'],
    &[b'<', b'v', b'>'],
];

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut solve = 0_usize;

    let mut data_in = data_in.lines();
    solve_p1(data_in.next().unwrap().as_bytes(), NUM_PAD);

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();
    let solve = 0;


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(solve),
        time_ms,
    }
}

fn solve_p1(input: &[u8], keypad: &[&[u8]]) {
    let mut pos = HashMap::new();
    for y in 0..keypad.len() {
        for x in 0..keypad[y].len() {
            if keypad[y][x] != b'x' {
                pos.insert(keypad[y][x], (x, y));
            }
        }
    }

    let mut sequences: HashMap<(u8, u8), Vec<Vec<u8>>> = HashMap::new();
    for (key, &(x, y)) in pos.iter() {
        for (key_2, &(x2, y2)) in pos.iter() {
            if key == key_2 {
                sequences.insert((*key, *key_2), vec![vec![b'A']]);
                continue;
            }
            let mut posibilities = vec![];
            let mut queue: VecDeque<(usize, usize, Vec<u8>)> = VecDeque::from([(x, y, vec![])]);
            let mut min = usize::MAX;
            'bfs: while let Some((px, py, path)) = queue.pop_front() {
                let px = px as i32;
                let py = py as i32;
                for (dx, dy, next_move) in [(1, 0, b'>'), (-1, 0, b'<'), (0, 1, b'v'), (0, -1, b'^')] {
                    if px + dx < 0 || py + dy < 0 || px + dx >= keypad[0].len() as i32 || py + dy >= keypad.len() as i32 {
                        continue;
                    }
                    if keypad[(py + dy) as usize][(px + dx) as usize] == b'x' {
                        continue;
                    }

                    let mut copy = path.clone();
                    copy.push(next_move);
                    if keypad[(dy + py) as usize][(dx + px) as usize] == *key_2 {
                        if min < copy.len() {break 'bfs}
                        min = copy.len();
                        copy.push(b'A');
                        posibilities.push(copy);
                    } else {
                        queue.push_back(((dx + px) as usize, (dy + py) as usize, copy));
                    }
            }
            }

            assert_ne!(posibilities.len(), 0);

            sequences.insert((*key, *key_2), posibilities);
        }
    }

    assert_eq!(sequences.get(&(b'2', b'9')).unwrap().len(), 3);
}
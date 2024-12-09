use crate::Solve;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn part1(data_in: &str) -> Solve {
    let time = Instant::now();
    let mut id = 0;
    let mut data: Vec<Option<u32>> = Vec::new();
    
    data_in
        .bytes()
        .map(|b| byte_char_to_num(b))
        .enumerate()
        .filter(|(_, b)| *b > 0)
        .for_each(|(idx, b)| {
            if idx % 2 == 0 {
                (0..b).for_each(|_| data.push(Some(id)));
                id += 1;
            } else {
                (0..b).for_each(|_| data.push(None));
            }
    });

    let mut i = 0;
    loop {
        if i >= data.len() {
            break;
        }

        if data[i].is_none() {
            while data.last().unwrap() == &None {
                data.pop();
            }

            data[i] = data.pop().unwrap();
        }

        i += 1;
    }
    let mut sum = 0;
    for (idx, value) in data.iter().flatten().enumerate() {
        sum +=  (*value as usize) * idx;
    }


    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(sum),
        time_ms,
    }
}

pub fn part2(data_in: &str) -> Solve {
    let time = Instant::now();

    let mut data = data_in.bytes()
        .map(|b| byte_char_to_num(b))
        .enumerate()
        .filter(|(_, b)| *b > 0)
        .map(|(idx, b)| {
            if idx % 2 == 0 {
                (Block::File(idx / 2), b as usize)
            } else {
                (Block::Empty, b as usize)
            }
        }).collect::<Vec<_>>();

    let mut idx = 0;
    loop {
        if idx >= data.len() {
            break;
        }

        if matches!(data[idx].0, Block::File(_)) {
            idx += 1;
            continue;
        }

        let size_of_empty = data[idx].1;
        for h in ((idx+1)..data.len()).rev() {
            if data[h].0 == Block::Empty {
                continue;
            }

            if size_of_empty >= data[h].1 {
                let to_insert = data[h];
                data[h].0 = Block::Empty;
                let original = data.get_mut(idx).unwrap();
                original.1 = original.1 - to_insert.1;
                data.insert(idx, to_insert);
                break;
            }
        }
        idx += 1;
    }

    let mut sum = 0;
    let mut i = 0;
    for (block, size) in data.into_iter().filter(|(_, s)| *s > 0) {
        match block {
            Block::Empty => {
                i += size;
            },
            Block::File(id) => {
                for num in i..size+i {
                    sum += id * num;
                }
                i += size;
            }
        }
    }

    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solve {
        solution: Box::new(sum),
        time_ms,
    }
}

fn byte_char_to_num(byte: u8) -> u8 {
    match byte {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(usize)
}
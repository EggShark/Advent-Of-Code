use crate::{Solution, SolutionType};
use std::fs::read_to_string;
use std::time::Instant;
use std::cmp::Ordering;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day13.txt").unwrap();
    let time = Instant::now();

    let mut packets = text.lines()
        .filter(|s| !s.is_empty())
        .map(|s| List::from_str(s))
        .collect::<Vec<List>>();

    let sol1 = packets.iter()
        .as_slice()
        .chunks(2)
        .enumerate()
        .fold(0, |sum, (i, pair)| {
            if let [l1, l2] = pair {
                if l1 < l2 {
                    sum + 1 + i
                } else {
                    sum
                }
            } else {
                unreachable!("stuff should be in pairs")
            }
    });
    packets.push(make_div(true));
    packets.push(make_div(false));
    packets.sort();
    let sol2 = (packets.iter().position(|p| *p == make_div(true)).unwrap() + 1) * (packets.iter().position(|p| *p == make_div(false)).unwrap() + 1);

    let solution = (SolutionType::USIZE(sol1), SolutionType::USIZE(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn make_div(kind: bool) -> List {
    if kind {
        List::from_str("[[2]]")
    } else {
        List::from_str("[[6]]")
    }
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum List {
    Parent(Vec<List>),
    Value(u8),
}

impl List {
    fn from_str(s: &str) -> Self {
        if s.starts_with('[') {
            let s = &s[1..s.len() - 1];
            let mut children:Vec<Self> = Vec::new();
            let mut depth = 0;
            let mut substring_start = 0;
            if s.len() == 0 {
                return Self::Parent(children);
            }
            for (i, char) in s.chars().enumerate() {
                match char {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' if depth == 0 => {
                        children.push(Self::from_str(&s[substring_start..i]));
                        substring_start = i + 1;
                    },
                    _ => {},
                }
            }
            children.push(List::from_str(&s[substring_start..]));
            Self::Parent(children)
        } else {
            Self::Value(s.parse().unwrap())
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.partial_cmp(b),
            (Self::Parent(children), Self::Parent(more_kids)) => children.partial_cmp(more_kids),
            (Self::Parent(_), Self::Value(a)) => self.partial_cmp(&Self::Parent(vec![Self::Value(*a)])),
            (Self::Value(a), List::Parent(children)) => vec![Self::Value(*a)].partial_cmp(children),
        }
    }
}
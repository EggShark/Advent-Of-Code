use crate::{Solution, SolutionType};
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("src/aoc2023/texts/day20").unwrap();
    let time = Instant::now();

//     let text = "broadcaster -> aa
// %aa -> in, co
// &in -> bb
// %bb -> co
// &co -> ou";

    let mut modules = parse_input(&text);

    let mut low_pulse_counter = 0;
    let mut high_pulse_counter = 0;

    let mut queue: VecDeque<Pulse> = VecDeque::new();
    for _ in 0..1000 {
        // send low pulse / press button we need to do this 100 times :pensive:

        //press the button
        low_pulse_counter += 1;
        let broadcast = modules.get(&[114, 111]).unwrap();
        for module_name in broadcast.connections.iter() {
            let pulse = Pulse {
                high: false,
                going_to: *module_name,
                from: [114, 111],
            };

            low_pulse_counter += 1;
            queue.push_back(pulse);
        }

        while let Some(pulse) = queue.pop_front() {
            let module = modules.get_mut(&pulse.going_to);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();

            let (lc, hc) = process_pulse(pulse, module, &mut queue);
            low_pulse_counter += lc;
            high_pulse_counter += hc;
        }
    }

    let mut modules = parse_input(&text);
    
    let feed_rx = modules
        .iter()
        .find(|(_, module)| module.connections.contains(&[114, 120]))
        .unwrap()
        .0.clone();

    let mut cycle_lenghts: HashMap<[u8; 2], u64> = HashMap::new();
    let mut seen: HashMap<[u8; 2], u64> = HashMap::new();
    for module in modules.iter() {
        if module.1.connections.contains(&feed_rx) {
            seen.insert(*module.0, 0);
        }
    }

    let mut queue: VecDeque<Pulse> = VecDeque::new();
    let mut loop_counter = 0;
    'a: loop {
        // send low pulse / press button we need to do this 100 times :pensive:

        //press the button
        loop_counter += 1;
        press_button(&mut queue, &modules);

        while let Some(pulse) = queue.pop_front() {
            let module = modules.get_mut(&pulse.going_to);

            if module.is_none() {
                continue;
            }

            let module = module.unwrap();

            if pulse.going_to == feed_rx && pulse.high {
                let seen_count = seen.get_mut(&pulse.from).unwrap();
                *seen_count += 1;

                if seen.values().all(|v| *v > 10) {
                    break 'a;
                }

                if !cycle_lenghts.contains_key(&pulse.from) {
                    cycle_lenghts.insert(pulse.from, loop_counter);
                }
            }

            process_pulse(pulse, module, &mut queue);
        }
    }

    let lcm = cycle_lenghts
        .into_values()
        .fold(1, |acc, num| lcm(acc, num));

    let sol1: u64 = low_pulse_counter * high_pulse_counter;
    let sol2: u64 = lcm;

    let solution = (SolutionType::U64(sol1), SolutionType::U64(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleKind {
    Brodcast,
    // high = true, low = false,
    Conjuction(HashMap<[u8; 2], bool>),
    // true = on false = off
    FlipFlop(bool)
}

#[derive(Debug)]
struct Module {
    connections: Vec<[u8; 2]>,
    kind: ModuleKind,
}

#[derive(Debug)]
struct Pulse {
    high: bool,
    going_to: [u8; 2],
    from: [u8; 2],
}

fn parse_input(text: &str) -> HashMap<[u8; 2], Module> {
    let mut modules = text
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let name_bytes = left.as_bytes();
            let module_kind = match name_bytes[0] {
                b'%' => ModuleKind::FlipFlop(false),
                b'&' => ModuleKind::Conjuction(HashMap::new()),
                _ => ModuleKind::Brodcast,
            };
            let connections = right.split(", ")
                .map(|cname| {
                    let c_bytes = cname.as_bytes();
                    [c_bytes[0], c_bytes[1]]
                })
                .collect::<Vec<[u8; 2]>>();

            let module = Module {
                kind: module_kind,
                connections,
            };

            ([name_bytes[1], name_bytes[2]], module)
        })
        .collect::<HashMap<[u8; 2], Module>>();

    // hella jank code to initlize module properly
    let connections = modules
        .iter()
        .map(|(name, module)| (name, &module.connections))
        .map(|(name, connections)| {
            (
                name, 
                connections
                    .iter()
                    .filter_map(|c_name| {
                        match modules.get(c_name) {
                            None => None,
                            Some(module) => Some((c_name, module)),
                        }
                    })
                    .filter(|(_, connection)| matches!(connection.kind, ModuleKind::Conjuction(_)))
                    .map(|(c_name, _)| *c_name)
                    .collect::<Vec<[u8; 2]>>()
            )
        })
        .filter(|(_, conj_con)| !conj_con.is_empty())
        .map(|(name, conj)| (*name, conj))
        .collect::<Vec<([u8; 2], Vec<[u8; 2]>)>>();

    for (name, conjunctors) in connections {
        for conj in conjunctors {
            let module = modules.get_mut(&conj).unwrap();
            match &mut module.kind {
                ModuleKind::Conjuction(map) => {
                    map.insert(name, false);
                }
                _ => unreachable!(),
            }
        }
    }

    modules
}

fn press_button(queue: &mut VecDeque<Pulse>, modules: &HashMap<[u8; 2], Module>) {
    let broadcast = modules.get(&[114, 111]).unwrap();
    for module_name in broadcast.connections.iter() {
        let pulse = Pulse {
            high: false,
            going_to: *module_name,
            from: [114, 111],
        };
        queue.push_back(pulse);
    }
}

fn process_pulse(pulse: Pulse, module: &mut Module, queue: &mut VecDeque<Pulse>) -> (u64, u64) {
    let mut counts = (0, 0);
    
    match &mut module.kind {
        ModuleKind::Conjuction(map) => {
            map.insert(pulse.from, pulse.high);
            // if all high pulses
            let high = !map.iter().all(|(_, v)| *v);
            let mut count = 0;
            module.connections.iter().for_each(|c| {
                queue.push_back(Pulse {
                    high,
                    going_to: *c,
                    from: pulse.going_to,
                });
                count += 1;
            });
            if high {
                counts.1 += count;
            } else {
                counts.0 += count;
            }
        },
        ModuleKind::FlipFlop(b) if !pulse.high => {
            // on to off, off to on
            *b = !*b;
            let mut count = 0;
            module.connections.iter().for_each(|c| {
                queue.push_back(Pulse {
                    high: *b,
                    going_to: *c,
                    from: pulse.going_to,
                });
                count += 1;
            });
            if *b {
                counts.1 += count;
            } else {
                counts.0 += count;
            }
        },
        _ => {},
    }

    counts
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a;
    let mut min = b;

    if min > max {
        std::mem::swap(&mut max, &mut min);
    }
    loop {
        let res = max % min;
        if res == 0 {
            return min
        }
        max = min;
        min = res;
    }
}
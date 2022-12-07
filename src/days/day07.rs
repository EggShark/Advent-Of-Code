use crate::{Solution, SolutionType};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::rc::Rc;
use std::time::Instant;
///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> Solution {
    // Your solution here...
    let text = read_to_string("./texts/day07.txt").unwrap();
    let time = Instant::now();
    let lines = text.lines();

    let root = Rc::new(RefCell::new(Node::new()));
    let mut current_node = Rc::clone(&root);
    for line in lines {
        let x = line.split_whitespace().collect::<Vec<&str>>();

        if line.starts_with('$') {
            current_node = parse_command(&x, &current_node, &root)
        } else if let [dir_size, name] = &x[..] {
            println!("{line}");
            parse_ls(dir_size, &current_node, name);
        }
    }
    let mut sizes = Vec::new();
    let rb = root.borrow();
    let (filled_space, sizes) = get_sum(&rb, &mut sizes);
    let sol1 = sizes.iter().filter(|num| **num < 100_000).sum::<u32>();
    let needed = 30000000 - (70000000 - filled_space);
    let sol2 = sizes.iter().filter(|num| **num > needed).min().unwrap();
    let sol1: u32 = sol1;
    let sol2: u32 = *sol2;

    let solution = (SolutionType::U32(sol1), SolutionType::U32(sol2));
    let time_ms = time.elapsed().as_nanos() as f64 / 1000000.0;
    Solution {
        solution,
        time_ms,
    }
}

fn get_sum<'a>(node: &Node, sizes: &'a mut Vec<u32>) -> (u32, &'a mut Vec<u32>) {
    if node.is_file {
        return (node.size.unwrap(), sizes);
    }
    let sum_c = node
        .children
        .values()
        .map(|child| get_sum(&child.borrow(), sizes).0)
        .sum();
    sizes.push(sum_c);
    (sum_c, sizes)
}

fn parse_command(tokens: &[&str], cur_node: &Rc<RefCell<Node>>, root: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    if tokens[1] != "cd" {
        return cur_node.clone();
    }
    let folder = tokens[2]; 
    match folder {
        ".." => Rc::clone(cur_node.borrow().parent.as_ref().unwrap()),
        "/" => root.clone(),
        _ => cur_node.borrow().children.get(folder).unwrap().clone()
    }
}

fn parse_ls(dir_size: &str, cur_node: &Rc<RefCell<Node>>, name: &str) {
    if !cur_node.borrow().children.contains_key(name) {
        instert_child(dir_size, cur_node, name);
    }
}

fn instert_child(dir_size: &str, cur_node: &Rc<RefCell<Node>>, name: &str) {
    let child = Rc::new(RefCell::new(Node::new()));
    let mut m_child = child.borrow_mut();
    if dir_size != "dir" {
        m_child.is_file = true;
        println!("{dir_size}");
        m_child.size = Some(dir_size.parse().unwrap());
    }
    m_child.parent = Some(Rc::clone(cur_node));
    cur_node.borrow_mut().children.insert(name.into(), Rc::clone(&child));
}

struct Node {
    size: Option<u32>,
    is_file: bool,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            size: None,
            is_file: false,
            children: HashMap::new(),
            parent: None,
        }
    }
}
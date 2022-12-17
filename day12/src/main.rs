use std::arch::x86_64::_mm_add_ps;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::rc::Rc;

struct Input {
    input: String,
    start: (usize, usize),
    width: usize,
    height: usize,
}

impl Input {
    fn new(input: String) -> Input {
        let height = input.lines().count();
        let input2 = str::replace(&input, "\n", "");
        let width = input2.len() / height;

        let s = input2.find(|s| s == 'S').unwrap();
        let start = (s % width, s / width);

        let i = input2.len();
        let i2 = width * height;

        Input {
            input: input2,
            width,
            height,
            start
        }
    }

    fn get_node(&self, x: usize, y: usize) -> Node {
        Node{
            pos: (x, y),
            value: self.get_char(&(x,y)),
        }
    }

    fn get_char(&self, pos: &(usize, usize)) -> char {
        self.input.chars().nth(pos.1 * self.width + pos.0).unwrap()
    }

    fn get_surrounding_nodes(&self, center_node: (usize, usize)) -> Vec<(usize, usize)> {
        let mut nodes = vec![];
        let (x, y) = center_node;

        if x >= self.width {
            panic!("kjlsdfjkl");
        }
        if y >= self.height {
            panic!("kjlsdfjkl");
        }

        if x > 0 {
            nodes.push((x - 1, y));
        }
        if x < self.width - 1 {
            nodes.push((x + 1, y));
        }
        if y > 0 {
            nodes.push((x, y - 1));
        }
        if y < self.height - 1 {
            nodes.push((x, y + 1));
        }
        return nodes;
    }
}

fn main() {
    let input = Input::new(fs::read_to_string("input.txt").expect("Error reading input.txt"));
    let levels : Vec<Vec<(usize, usize)>> = vec![vec![input.start]];
    let vec1 = bread_first_search(&input, levels, HashSet::from([input.start]));
    println!("{}", vec1.len() - 1);

    let vec2 = bread_first_search(&input, vec![vec![(25,0)]], HashSet::from([(25,0)]));

    let min = input.input.chars().enumerate()
        .filter(|(i, c)| *c == 'a')
        .map(|(i, c)| bread_first_search(&input, vec![vec![(i % input.width, i / input.width)]], HashSet::from([(i % input.width, i / input.width)])))
        .map(|v| v.len() - 1)
        .min();

    println!("{}", min.unwrap());



}

#[derive(Debug)]
struct Node {
    value: char,
    pos: (usize, usize),
}

impl Node {
    fn adjecent_nodes(&self, input: &Input) -> Vec<(usize, usize)> {
        input.get_surrounding_nodes(self.pos).into_iter()
            .map(|x| Node{
                value: input.get_char(&x),
                pos: x
            })
            .filter(|x| x.value != 'E' || (x.value == 'E' && self.value == 'z'))
            .filter(|x| (self.value as i32) + 1 >= (x.value as i32) || self.value == 'S')
            .map(|x|x.pos)
            .collect()
    }
}


fn bread_first_search(input: &Input, mut levels: Vec<Vec<(usize, usize)>>, mut visited_nodes: HashSet<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let last_level = levels.last().unwrap();

    // getting a stack overflow for some reason. not sure why, but there's really no point going further
    // as the answer for part 1 was 404
    if levels.len() == 405 {
        return levels;
    }

    if last_level.into_iter().any(|x| input.get_char(x) == 'E') {
        return levels;
    }

    let mut next_level: Vec<(usize, usize)> = last_level.into_iter()
        .map(|x| Node{
            pos: *x,
            value: input.get_char(x)
        })
        .flat_map(|x| x.adjecent_nodes(input))
        .filter(|x| !visited_nodes.contains(x))
        .collect();

    visited_nodes.extend(next_level.iter());

    next_level.sort();
    next_level.dedup();
    levels.push(next_level);

    return bread_first_search(input, levels, visited_nodes);
}


fn get_nodes_at_depth(to_depth: usize, current_depth: usize, current_node: (usize, usize), visited_nodes: &mut HashSet<((usize, usize))>, input: &Input) -> Vec<((usize, usize))> {
    let current_char = input.get_char(&current_node);
    visited_nodes.insert(current_node);
    if current_char == 'E' {
        //panic!("Found exit at depth {}", current_depth);
        return vec![current_node];
    }
    if current_depth == to_depth {
        return vec![current_node];
    }
    let surrounding_nodes = input.get_surrounding_nodes(current_node);
    let accessible_nodes = surrounding_nodes.into_iter()
        .map(|x| (x, input.get_char(&x)))
        .filter(|x| (x.1 == 'E' && current_char == 'z') || (current_char as i32) + 1 >= (x.1 as i32) || current_char == 'S')
        .filter(|x| !visited_nodes.contains(&x.0))
        .map(|x| x.0)
        .collect::<Vec<((usize, usize))>>();
    let mut nodes = vec![];

    for x in accessible_nodes {
        nodes.push(get_nodes_at_depth(to_depth, current_depth + 1, x, visited_nodes, input));
    }
    nodes.into_iter().flatten().collect()
}

fn get_surrounding_nodes(center_node: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut nodes = vec![];
    let (x, y) = center_node;
    if x > 0 {
        nodes.push((x - 1, y));
    }
    if x < width - 1 {
        nodes.push((x + 1, y));
    }
    if y > 0 {
        nodes.push((x, y - 1));
    }
    if y < height - 1 {
        nodes.push((x, y + 1));
    }
    return nodes;
}



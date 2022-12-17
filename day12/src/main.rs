use std::arch::x86_64::_mm_add_ps;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::rc::Rc;
use std::thread::available_parallelism;

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
    println!("Levels: {}", levels.len());
    let last_level = levels.last().unwrap();

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

fn breadth_first_search2(current_depth: i32, max_depth: i32, current_node: (usize, usize), mut visited: HashSet<(usize, usize)>, input: &String) -> Option<i32> {

    if visited.contains(&current_node) {
        return None;
    }
    visited.insert(current_node);

    let height = input.lines().count();
    let width = 1 + input.len() / height;
    let c = input.chars().nth(current_node.0 + current_node.1 * width).unwrap();

    let option = input.find(|c| c == 'E').unwrap();

    let aa= option / width;
    let bb= option - aa*width;

    if c == 'E' {
        return Some(current_depth);
    }
    if max_depth == current_depth {
        return None;
    }

    let surrounding_nodes = get_surrounding_nodes(current_node, width, height);

    let mut min_depth: Option<i32> = None;
    for node in surrounding_nodes {
        if visited.contains(&node) {
            continue;
        }
        let cc = input.chars().nth(node.0 + node.1 * width).expect(format!("Node {} is out of bounds", node.0 + node.1 * 9).as_str());
        if c == 'z' && cc == 'E'{
            let a = 123;
            return Some((current_depth+1));
        } else if cc == 'E' {
            continue;
        }

        if cc == '\n' {
            let a = 123;
            continue;
        }
        if cc as i32 > (c as i32) + 1 {
            let a = 123;
            continue;
        }
        let result = breadth_first_search2(current_depth + 1, max_depth, node, visited.clone(), input);

        if result.is_some() && min_depth == None {
            min_depth = Some(result.unwrap());
        } else if result.is_some() && result.unwrap() < min_depth.unwrap() {
            min_depth = Some(result.unwrap());
        }
    }
    min_depth


//            if cc as i32 <= (c as i32) + 1 {

    //let tests = surrounding_nodes.iter()
    //    .filter(|n| **n != previous_node)
    //    .filter(|n| {
    //        let cc = input.chars().nth(n.0 + n.1 * width).unwrap();
    //        cc == 'E' || cc as i32 <= (c as i32) + 1
    //    })
    //    .inspect(|n| {
    //    })
    //    .map(|n| breadth_first_search(current_depth + 1, max_depth, *n,current_node, input))
    //    .filter(|n| n.is_some())
    //    .map(|n| n.unwrap())
    //    .min();
    //match tests {
    //    Some(n) => Some(n),
    //    None => None,
    //}
}





//fn main() {
//
//    let nn = Node { value: 'a', nodes: vec![] };
//    let nnn = Node { value: 'a', nodes: vec![] };
//
//    let mut n = Rc::from(RefCell::new(nn));
//    let mut n1 = Rc::from(RefCell::new(nnn));
//
//    let mut map: HashMap<(u32, u32),Rc<RefCell<Node>>> = HashMap::new();
//    map.insert((1,2),n.clone());
//    map.insert((2,2),n1.clone());
//
//    n.borrow_mut().nodes.push(n1.clone());
//}

//fn parse_input(input: &str) -> Result<Rc<RefCell<Node>>, Box<dyn Error>> {
//    let mut map: HashMap<(usize, usize),Rc<RefCell<Node>>> = HashMap::new();
//    let input = fs::read_to_string(input)?;
//
//    let height = input.lines().count();
//    let width = input.len() / height;
//
//    let start_node = Rc::from(RefCell::new(Node { value: input.chars().nth(0).unwrap(), nodes: vec![] }));
//    map.insert((0,0), start_node.clone());
//
//    for i in 0..input.len() {
//        let y = i / width;
//        let x = i - (y * width);
//        let c = input.chars().nth(i).unwrap();
//
//        let node = match map.get_mut(&(x, y)) {
//            None => &mut Rc::from(RefCell::new(Node { value: c, nodes: vec![] })),
//            Some(x) => x
//        };
//
//        if c == 'E' {
//            continue;
//        }
//
//        let surrounding_nodes = get_surrounding_nodes((x,y), width, height);
//
//        for (x, y) in surrounding_nodes {
//            let cc = input.chars().nth(x+y*width).unwrap();
//            if cc as i32 <= (c as i32) + 1 {
//                let adjacent_node = match map.get_mut(&(x, y)) {
//                    None => &mut Rc::from(RefCell::new(Node { value: cc, nodes: vec![] })),
//                    Some(x) => x
//                };
//
//                node.borrow_mut().nodes.push(adjacent_node.clone());
//            }
//        }
//    }
//    Ok(start_node.clone())
//}
//

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



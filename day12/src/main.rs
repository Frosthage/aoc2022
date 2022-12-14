extern crate core;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::format;
use std::fs;
use std::rc::Rc;

struct Node {
    value: char,
    nodes: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut depth = 1;

    loop {
        depth += 1;
        println!("Depth: {}", depth);
        let nodes: Vec<((usize, usize))> = Vec::new();
        match breadth_first_search(1, depth, (0, 0), nodes, &input) {
            None => {}
            Some(x) => {println!("Found: {}", x); break;}
        }
    }
}

fn breadth_first_search(current_depth: i32, max_depth: i32, current_node: (usize, usize), mut visited: Vec<(usize, usize)>, input: &String) -> Option<i32> {

    if visited.contains(&current_node) {
        return None;
    }
    visited.push(current_node);

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
        let result = breadth_first_search(current_depth + 1, max_depth, node, visited.clone(), input);

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
    if x < width - 2 {
        nodes.push((x + 1, y));
    }
    if y > 0 {
        nodes.push((x, y - 1));
    }
    if y < height - 1 {
        nodes.push((x, y + 1));
    }
    let o = 123;
    return nodes;
}



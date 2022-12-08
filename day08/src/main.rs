use std::cmp::{min, max};
use std::error::Error;
use std::fs;

fn get_tree_lines(x:usize, y:usize, tree: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut left = tree[y][0..x].to_vec();
    left.reverse();
    let right = tree[y][x+1..].to_vec();
    let mut up = Vec::new();
    let mut down = Vec::new();
    for i in 0..y {
       up.push(tree[i][x]);
    }
    up.reverse();
    for i in y+1..tree.len() {
        down.push(tree[i][x]);
    }
    [left, right, up, down].to_vec()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let vec = input.lines()
        .map(|x| Vec::from(x.chars().map(|y| y.to_digit(10).unwrap()).collect::<Vec<u32>>())).collect::<Vec<Vec<u32>>>();


    part1(&vec);
    part2(&vec);

    Ok(())
}


fn part2(vec: &Vec<Vec<u32>>) {
    let mut highest_scenic_score = 0;
    for y in 1..vec.len() - 1 {
        for x in 1..vec[0].len() - 1 {
            let current_tree = vec[y][x];
            let tree_lines = get_tree_lines(x, y, &vec);

            let scenic_score =
                min(tree_lines[0].len(), tree_lines[0].iter().take_while(|y| **y < current_tree).count()+1) *
                min(tree_lines[1].len(), tree_lines[1].iter().take_while(|y| **y < current_tree).count()+1) *
                min(tree_lines[2].len(), tree_lines[2].iter().take_while(|y| **y < current_tree).count()+1) *
                min(tree_lines[3].len(), tree_lines[3].iter().take_while(|y| **y < current_tree).count()+1);

            highest_scenic_score = max(highest_scenic_score, scenic_score);
        }
    }
    println!("Part 2: {}", highest_scenic_score);

}

fn part1(vec: &Vec<Vec<u32>>) {

    let mut count = vec.len() + vec.len() + vec[0].len() -2 + vec[0].len() - 2;
    for y in 1..vec.len() - 1 {
        for x in 1..vec[0].len() - 1 {
            let current_tree = vec[y][x];
            let tree_lines = get_tree_lines(x, y, &vec);
            if tree_lines.iter().any(|x| x.iter().all(|y| current_tree > *y)) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

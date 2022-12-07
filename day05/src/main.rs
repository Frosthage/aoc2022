use std::{fs, io};
use std::error::Error;
use std::fs::create_dir;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {

    let mut v1:Vec<i32> = (1..10).collect();
    let mut v2:Vec<i32> = Vec::new();

    let mut d = v1.drain((3..7)).collect::<Vec<i32>>();
    v2.append(&mut d);
    Ok(())

}

fn Part2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut split = input.split("\n\n");
    let crates = split.next().ok_or("No crates")?;
    let instructions = split.next().ok_or("No instructions")?;

    let rows = crates.split('\n')
        .rev()
        .skip(1);

    let mut crate_stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let re = Regex::new(r"([A-Z])")?;
    for row in rows {
        for c in re.captures_iter(row) {
            let m = c.get(0).ok_or("No match")?;
            let start = m.start();
            let index = (start - 1) / 4;
            crate_stacks[index].push(m.as_str().chars().next().unwrap());
        }
    }

    let re2 = Regex::new(r"move (\d*?) from (\d) to (\d)")?;
    for i in instructions.trim().split('\n') {
        for c in re2.captures_iter(i) {
            let m = c.get(1).ok_or("No match")?;
            let f = c.get(2).ok_or("No match")?;
            let t = c.get(3).ok_or("No match")?;
            let index1 = m.as_str().parse::<usize>()?;
            let index2 = f.as_str().parse::<usize>()? - 1;
            let index3 = t.as_str().parse::<usize>()? - 1;

            let start = &crate_stacks[index2].len() - index1;

            let mut stack = crate_stacks[index2].drain(start..).collect::<Vec<char>>();
            crate_stacks[index3].append(&mut stack);
        }
    }

    for mut c in crate_stacks {
        match c.pop() {
            None => {}
            Some(c) => print!("{}", c)
        }
    }


    Ok(())
}


fn Part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut split = input.split("\n\n");
    let crates = split.next().ok_or("No crates")?;
    let instructions = split.next().ok_or("No instructions")?;

    let rows = crates.split('\n')
        .rev()
        .skip(1);

    let mut crate_stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let re = Regex::new(r"([A-Z])")?;
    for row in rows {
        for c in re.captures_iter(row) {
            let m = c.get(0).ok_or("No match")?;
            let start = m.start();
            let index = (start - 1) / 4;
            crate_stacks[index].push(m.as_str().chars().next().unwrap());
        }
    }

    let re2 = Regex::new(r"move (\d*?) from (\d) to (\d)")?;
    for i in instructions.trim().split('\n') {
        for c in re2.captures_iter(i) {
            let m = c.get(1).ok_or("No match")?;
            let f = c.get(2).ok_or("No match")?;
            let t = c.get(3).ok_or("No match")?;
            let index1 = m.as_str().parse::<usize>()?;
            let index2 = f.as_str().parse::<usize>()? - 1;
            let index3 = t.as_str().parse::<usize>()? - 1;


            for i in 0..index1 {
                let c = crate_stacks[index2].pop().ok_or("stack is empty")?;
                crate_stacks[index3].push(c);
            }
        }
    }

    for mut c in crate_stacks {
        match c.pop() {
            None => {}
            Some(c) => print!("{}", c)
        }
    }


    Ok(())
}

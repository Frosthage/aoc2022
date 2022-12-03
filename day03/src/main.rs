use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    part2()
}

fn part2()  -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let prio: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let strings: Vec<&str> = input.trim()
        .split('\n')
        .collect();

    let sum: usize = strings.chunks(3)
        .map(|x| {
            let s0: HashSet<char> = x[0].chars().collect();
            let s1: HashSet<char> = x[1].chars().collect();
            let s2: HashSet<char> = x[2].chars().collect();
            //let i1: HashSet<char> = s0.intersection(&s1);
            let i1: HashSet<char> = s0.intersection(&s1).map(|x| x.clone()).collect();
            let i2: String = i1.intersection(&s2).take(1).collect();
            i2.chars().next().unwrap()
        })
        .map(|x| prio.get(&x).unwrap())
        .sum();
    println!("{}", sum);
    Ok(())
}

fn part1()  -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let prio: HashMap<char, usize> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    //let apa: Vec<char> = input.trim().split('\n')
    let sum: usize = input.trim().split('\n')
        .map(|x| x.split_at(x.len() / 2))
        .map(|x| {
            let s0: HashSet<char> = x.0.chars().collect();
            let s1: HashSet<char> = x.1.chars().collect();
            let intersect: String = s0.intersection(&s1).take(1).collect();
            intersect.chars().next().unwrap()
        })
        .map(|x| prio.get(&x).unwrap())
        .sum();
    println!("{}", sum);
    Ok(())

}










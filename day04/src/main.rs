use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::ops::{Not, Range};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1: {}", solve(|s1,s2| s1.is_subset(&s2) || s2.is_subset(&s1))?);
    println!("Part 2: {}", solve(|s1,s2| s1.is_disjoint(&s2).not())?);
    Ok(())
}

fn solve(filterPredicate: fn(s1: &HashSet<i32>, s2: &HashSet<i32>) -> bool) -> Result<usize, Box<dyn Error>> {
    let input = fs::read_to_string("input")?;
    let count = input
        .split('\n')
        .map(|x| x.split(',').collect::<Vec<&str>>())
        .map(|x| {
            let section1 = x[0].split('-').collect::<Vec<&str>>();
            let section2 = x[1].split('-').collect::<Vec<&str>>();
            (
                (section1[0].parse::<i32>().unwrap()..(section1[1].parse::<i32>().unwrap()+1)),
                (section2[0].parse::<i32>().unwrap()..(section2[1].parse::<i32>().unwrap()+1)),
            )
        })
        .filter(|x| {
            let s1:HashSet<i32> = HashSet::from_iter(x.0.clone());
            let s2:HashSet<i32> = HashSet::from_iter(x.1.clone());
            filterPredicate(&s1,&s2)
        })
        .count();
    Ok(count)

}


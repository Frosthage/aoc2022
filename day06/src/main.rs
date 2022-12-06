use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    Part2()

}

fn Part2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    for i in 0..input.len() {
        let substring = &input[(i..i + 14)];
        let set = substring.chars().collect::<HashSet<char>>();
        if set.len() == 14 {
            println!("{}", i + 14);
            break;
        }
    }

    Ok(())
}

fn Part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    for i in 0..input.len() {
        let substring = &input[(i..i + 4)];
        let set = substring.chars().collect::<HashSet<char>>();
        if set.len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }

    Ok(())
}

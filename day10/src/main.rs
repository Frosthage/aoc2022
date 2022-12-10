use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum instruction {
    noop,
    addx(i32),
}


fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let map: Vec<instruction> = input
        .lines()
        .map(|line| {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["noop"] => instruction::noop,
                ["addx", x] => instruction::addx(x.parse().unwrap()),
                _ => panic!("Invalid instruction"),
            }
        }).collect();

    let mut ticks = Vec::new();
    let mut x = 1;

    for i in map {
        match i {
            instruction::noop => {
                ticks.push(x);
            },
            instruction::addx(y) => {
                ticks.push( x);
                ticks.push( x);
                x += y;
            }
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            let p = x + y * 40;
            let sprite = [ticks[p] - 1, ticks[p], ticks[p] + 1];
            if sprite.contains(&(x as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    Ok(())
}



fn part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let map: Vec<instruction> = input
        .lines()
        .map(|line| {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["noop"] => instruction::noop,
                ["addx", x] => instruction::addx(x.parse().unwrap()),
                _ => panic!("Invalid instruction"),
            }
        }).collect();

    let mut ticks = Vec::new();
    let mut x = 1;

    for i in map {
        match i {
            instruction::noop => {
                ticks.push(x);
            },
            instruction::addx(y) => {
                ticks.push(x);
                ticks.push(x);
                x += y;
            }
        }
    }

    let measure = HashSet::from([19, 59, 99, 139, 179, 219]);

    let part1: i32 = ticks.iter().enumerate()
        .filter(|(i, &x)| measure.contains(i))
        .map(|(i, &x)| (i as i32, x))
        .map(|(i, x)| x * (i + 1))
        .sum();

    println!("{:?}", part1);

    Ok(())
}



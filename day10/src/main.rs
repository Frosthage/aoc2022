use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}


fn main() -> Result<(), Box<dyn Error>> {
    let register_values = get_register_values()?;

    part1(&register_values)?;
    part2(&register_values)?;
    Ok(())
}

fn get_register_values() -> Result<Vec<i32>, Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let instructions= input
        .lines()
        .map(|line| {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                ["noop"] => Instruction::Noop,
                ["addx", x] => Instruction::Addx(x.parse().unwrap()),
                _ => panic!("Invalid instruction"),
            }
        });

    let mut register_values = Vec::new();
    let mut x_register = 1;

    for i in instructions {
        match i {
            Instruction::Noop => {
                register_values.push(x_register);
            },
            Instruction::Addx(x) => {
                register_values.push(x_register);
                register_values.push(x_register);
                x_register += x;
            }
        }
    }
    Ok(register_values)
}


fn part1(register_values: &Vec<i32>) -> Result<(), Box<dyn Error>> {

    let measure = HashSet::from([19, 59, 99, 139, 179, 219]);

    let part1: i32 = register_values.iter()
        .enumerate()
        .filter(|(i, _)| measure.contains(i))
        .map(|(i, &x)| (i as i32, x))
        .map(|(i, x)| x * (i + 1))
        .sum();

    println!("{:?}", part1);

    Ok(())
}

fn part2(register_values: &Vec<i32>) -> Result<(), Box<dyn Error>> {

    for y in 0..6 {
        for x in 0..40 {
            let p = x + y * 40;
            let sprite = [register_values[p] - 1, register_values[p], register_values[p] + 1];
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

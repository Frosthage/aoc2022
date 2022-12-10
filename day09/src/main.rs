extern crate core;

use std::collections::{HashSet};
use std::error::Error;
use std::{fmt, fs};
use std::process::Command;

struct Vector {
    x: i32,
    y: i32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl Copy for Position {

}

fn parse_input(input: &str) -> Vec<(Vector, u32)> {
    input.lines()
        .map(|line| line.split(' '))
        .map(|line| {
        match line.collect::<Vec<&str>>()[..] {
            ["R", x] => (Vector{x:1, y:0}, x.parse().unwrap()),
            ["L", x] => (Vector{x:-1, y:0}, x.parse().unwrap()),
            ["U", x] => (Vector{x:0, y:1}, x.parse().unwrap()),
            ["D", x] => (Vector{x:0, y:-1}, x.parse().unwrap()),
            _ => panic!("Invalid input"),
        }
        })
        .collect::<Vec<(Vector,u32)>>()
}


fn main() -> Result<(), Box<dyn Error>>{
    part1()?;
    part2()?;
    Ok(())

}

fn part2() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let movements = parse_input(&input);

    let mut knots =  [Position{x:10,y: 10}; 10];

    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    for m in movements {
        for _ in 0..m.1 {
            knots[0] = Position { x: knots[0].x + m.0.x, y: knots[0].y + m.0.y };

            for i in 0..9 {
                let mut k = &mut knots[i..i+2];
                let diff_x = k[0].x - k[1].x;
                let diff_y = k[0].y - k[1].y;
                if diff_x.abs() < 2 && diff_y.abs() < 2 {
                    break;
                }
                let norm1 = norm(diff_x, diff_y);
                k[1].x = k[1].x + norm1.0;
                k[1].y = k[1].y + norm1.1;
            }

            positions.insert((knots.last().unwrap().x, knots.last().unwrap().y));
        }
    }

    println!("{:?}", positions.len());
    Ok(())
}

fn norm(px: i32, py: i32) -> (i32,i32) {
    let mut x = px;
    let mut y = py;
    if px != 0 {
        x = px / px.abs();
    }
    if py != 0 {
        y = py / py.abs();
    }
    (x,y)
}

fn print_rope(knots: &[Position; 10], width: i32, height: i32) {
    Command::new("cls");
    for y in (0..height).rev() {
        for x in 0..width {
            let c = match knots.iter().enumerate().position(| k| k.1.x == x && k.1.y == y) {
                Some(i) if i == 0 => "H".to_string(),
                Some(i) => i.to_string(),
                None => ".".to_string(),
            };

            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn part1() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let movements = parse_input(&input);

    let mut head: Position = Position { x: 0, y: 0 };
    let mut tail: Position = Position { x: 0, y: 0 };

    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert((0, 0));
    for x in movements {
        for _ in 1..x.1 + 1 {
            head = Position { x: head.x + x.0.x, y: head.y + x.0.y };

            let diff_x = head.x - tail.x;
            let diff_y = head.y - tail.y;

            if diff_x.abs() == 2 || diff_y.abs() == 2 {
                tail = Position { x: head.x - x.0.x, y: head.y - x.0.y };
            }

            positions.insert((tail.x, tail.y));
        }
    }

    println!("{:?}", positions.len());

    Ok(())
}

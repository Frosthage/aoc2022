
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let points: u32 = fs::read_to_string("input.txt")?
        .trim()
        .split('\n')
        .map(|x| x
            .replace("A","R")
            .replace("B","P")
            .replace("C","S"))
        .map(|x|
            match x.as_str() {
                "R X" => "R S",
                "R Y" => "R R",
                "R Z" => "R P",
                "P X" => "P R",
                "P Y" => "P P",
                "P Z" => "P S",
                "S X" => "S P",
                "S Y" => "S S",
                "S Z" => "S R",
                _ => panic!("{}", x)
            }
        )
        .map(|x|
            match x {
                "R R" => 3+1,
                "P R" => 0+1,
                "S R" => 6+1,
                "R P" => 6+2,
                "P P" => 3+2,
                "S P" => 0+2,
                "R S" => 0+3,
                "P S" => 6+3,
                "S S" => 3+3,
                _ => panic!("{}", x)

        })
        .sum();

    println!("{}", points);
    Ok(())

}

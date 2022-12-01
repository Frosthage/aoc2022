use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::slice::Split;
use std::cmp;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let file = File::open("input.txt")?;
    let mut br = BufReader::new(file);
    br.read_to_string(&mut input)?;

    let mut current:u32 = 0;
    let mut array = [0;4];

    for a in input.split(|x| x == '\n') {
        if !a.is_empty() {
            current += a.parse::<u32>()?;
        } else {
            array[0] = current;
            current=0;
            array.sort();
        }
    }
    let apa = &array[1..];

    println!("{}", &array[1..].iter().sum::<u32>());

    Ok(())
}

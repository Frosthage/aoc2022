use std::error::Error;
use std::fs;
use regex::Regex;

enum Operation{
    Mul(u64),
    Add(u64),
    Pow(),
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    ifTrue: usize,
    ifFalse: usize,
    items_inspected: usize,
}

fn main()-> Result<(), Box<dyn Error>> {
    let mut monkeys = parse_monkeys("input.txt")?;
    part1(monkeys);
    let mut monkeys = parse_monkeys("input.txt")?;
    part2(monkeys);

    Ok(())
}

fn part2(mut monkeys: Vec<Monkey>) {
    let product = monkeys.iter().map(|m| m.test).reduce(|a, b| a * b).unwrap();
    for j in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i].items_inspected += monkeys[i].items.len();
            let mut items: (Vec<u64>, Vec<u64>) = monkeys[i].items.iter()
                .map(|item|
                    match monkeys[i].operation {
                        Operation::Mul(x) => item * x,
                        Operation::Add(x) => item + x,
                        Operation::Pow() => item * item
                    })
                //.inspect(|x| println!("{} % {} = {}", monkeys[i].test, x, monkeys[i].test % x))
                .map(|x| x % product)
                .partition(|item| item % monkeys[i].test == 0);

            let ifTrue = monkeys[i].ifTrue;
            let ifFalse = monkeys[i].ifFalse;
            monkeys[i].items.clear();
            monkeys[ifTrue].items.append(&mut items.0);
            monkeys[ifFalse].items.append(&mut items.1);
        }
    }

    let mut vec = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<usize>>();
    vec.sort();
    println!("{:?}", vec.into_iter().rev().take(2).reduce(|a, b| a * b));
}


fn part1(mut monkeys: Vec<Monkey>) {
    for j in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].items_inspected += monkeys[i].items.len();
            let mut items: (Vec<u64>, Vec<u64>) = monkeys[i].items.iter()
                .map(|item|
                    match monkeys[i].operation {
                        Operation::Mul(x) => item * x,
                        Operation::Add(x) => item + x,
                        Operation::Pow() => item * item
                    })
                .map(|x| x / 3)
                //.inspect(|x| println!("{} % {} = {}", monkeys[i].test, x, monkeys[i].test % x))
                .partition(|item| item % monkeys[i].test == 0);

            let ifTrue = monkeys[i].ifTrue;
            let ifFalse = monkeys[i].ifFalse;
            monkeys[i].items.clear();
            monkeys[ifTrue].items.append(&mut items.0);
            monkeys[ifFalse].items.append(&mut items.1);
        }
    }

    let mut vec = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<usize>>();
    vec.sort();
    println!("{:?}", vec.into_iter().rev().take(2).reduce(|a, b| a * b));
}

fn parse_monkeys(path: &str) -> Result<Vec<Monkey>, Box<dyn Error>> {
    let mut result: Vec<Monkey> = Vec::new();
    let string = fs::read_to_string(path)?;
    let input = string.split("\n\n");

    let rx_starting_items = Regex::new(r"Starting items: (.*)")?;
    let rx_operation = Regex::new(r"new = (.*)")?;

    for m in input {
        //skip first line, Monkey N:
        let mut starting_items: Vec<u64> = Vec::new();
        let mut monkey = m.lines().skip(1);

        let starting_items_str = rx_starting_items
            .captures(monkey.next().unwrap())
            .unwrap().get(1).unwrap().as_str();

        for i in starting_items_str.split(", ") {
            starting_items.push(i.parse::<u64>().unwrap());
        }
        let operation = rx_operation
            .captures(monkey.next().unwrap())
            .unwrap().get(1).unwrap().as_str();
        let o = match operation.split(" ").collect::<Vec<&str>>()[..] {
            ["old", "*", "old"] => Operation::Pow(),
            ["old", "+", "old"] => Operation::Mul(2),
            ["old", "*", x] => Operation::Mul(x.parse::<u64>().unwrap()),
            ["old", "+", x] => Operation::Add(x.parse::<u64>().unwrap()),
            _ => panic!("Invalid operation"),
        };
        let test = monkey.next().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse::<u64>().unwrap();
        let if_true = monkey.next().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();
        let if_false = monkey.next().unwrap().split(' ').collect::<Vec<&str>>().last().unwrap().parse::<usize>().unwrap();

        if let Some(_) = monkey.next() {
            Err("Invalid monkey")?;
        }

        result.push(Monkey {
            items: starting_items,
            operation: o,
            test: test,
            ifTrue: if_true,
            ifFalse: if_false,
            items_inspected: 0,
        });
    }

    Ok(result)


}

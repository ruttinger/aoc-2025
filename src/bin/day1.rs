use std::io; 
use std::io::Read; 

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = parse(&input);
    println!("{}",result)
}

fn parse(input: &String) -> String {
    let lines = input.split("\n");
    let mut dial = 50;
    let mut counter = 0;
    for line in lines {
        dial = turn(dial, interpret(line));

        if dial == 0 {
            counter += 1;
        }
    }

    return counter.to_string();
}

fn interpret(input: &str) -> i32 {
    let amount: i32;
    if input.contains("R") {
        amount = input[1..].parse().unwrap();
        return amount;
    } else if input.contains("L") {
        amount = input[1..].parse().unwrap();
        return -amount;
    }
    return 0;
}

fn turn(dial: i32, amount: i32) -> i32 {
    let raw = dial + amount;

    let result = ((raw % 100) + 100) % 100;

    result
}

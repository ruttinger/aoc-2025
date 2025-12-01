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
        let (new_dial, corssings) = turn(dial, interpret(line));
        dial = new_dial;

        counter += crossings;
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

fn turn(dial: i32, amount: i32) -> (i32, i32) {
    if amount == 0 {
        return (dial, 0);
    }

    let mut crossings = 0;
    let mut current = dial;

    let step = if amount > 0 { 1 } else { -1 };

    for _ in 0..amount.abs() {
        current += step;

        // wrap around
        if current == 100 {
            current = 0;
        } else if current == -1 {
            current = 99;
        }

        // count hitting 0
        if current == 0 {
            crossings += 1;
        }
    }

    (current, crossings)
}

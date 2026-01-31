use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", parse(&input));
}

fn parse(input: &str) -> u64 {
    let primops: Vec<&str> = input
        .lines()
        .last()
        .expect("primitive op!")
        .split_whitespace()
        .collect();

    let lines: Vec<&str> = input.lines().collect();

    let mut result: Vec<u64> = vec![0; primops.len()];

    for (j, op) in primops.iter().enumerate() {
        if *op == "*" {
            result[j] = 1;
        }
    }

    for line in lines.iter().take(lines.len().saturating_sub(1)) {
        for (j, num) in line.split_whitespace().enumerate() {
            let num: u64 = num.parse().expect("invalid number");

            match primops[j] {
                "+" => result[j] += num,
                "*" => result[j] *= num,
                _ => panic!("unknown operator"),
            }
        }
    }

    result.iter().sum()
}

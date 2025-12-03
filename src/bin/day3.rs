use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", parse(&input));
}

fn parse(input: &str) -> u32 {
    let mut total_joltage = 0;

    for line in input.lines() {
        let joltage = interpret(line);
        total_joltage += joltage;
    }

    total_joltage
}

fn interpret(line: &str) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let mut highest: u32 = 0;
    let mut second: u32 = 0;

    for i in 0..chars.len() {
        if let Some(n) = chars[i].to_digit(10) {
            if i < chars.len()-1 {
                // only change n here
                if n > highest {
                    highest = n;
                    second = 0;
                } else if n > second {
                    second = n;
                }
            } else {
                if n > second {
                    second = n;
                }
            }
        }
    }

    highest * 10 + second
}

use std::io; 
use std::io::Read; 

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    let result = parse(&input);
    println!("{}", result);
}

fn parse(input: &String) -> i64 {
    let ranges = input.split(",");
    let mut invalid_sum: i64 = 0;

    for range in ranges {
        let Some((start, end)) = range.split_once("-") else {
            continue;
        };

        let (Ok(i), Ok(j)) = (start.trim().parse::<i64>(), end.trim().parse::<i64>()) else {
            continue;
        };

        invalid_sum += interpret(i, j);
    }

    invalid_sum
}

fn interpret(start: i64, end: i64) -> i64 {
    let mut result: i64 = 0;
    for i in start..=end {
        let invalid = repeating(i);
        if invalid > 0 {
            result += invalid;
        }
    }
    result
}

fn repeating(num: i64) -> i64 {
    let s = num.to_string();
    let doubled = s.clone() + &s;

    let inner = &doubled[1..doubled.len() - 1];

    if inner.contains(&s) {
        num
    } else {
        0
    }
}

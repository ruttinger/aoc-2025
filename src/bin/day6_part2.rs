use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", parse(&input));
}

fn parse(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();

    // collect columns as strings
    let mut columns: Vec<String> = vec![String::new(); width];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
    }

    let mut total = 0u128;
    let mut current_sum = 0u128;
    let mut current_op = '+';

    for col in columns {
        let trimmed = col.trim();

        // prepare for calculation later
        if trimmed.ends_with('+') || trimmed.ends_with('*') {
            total += current_sum;
            current_op = trimmed.chars().last().unwrap();
            current_sum = match current_op {
                '+' => 0,
                '*' => 1,
                _ => 0,
            };
        }

        let num_str: String = trimmed.chars().filter(|c| c.is_ascii_digit()).collect();
        if !num_str.is_empty() {
            let num = num_str.parse::<u128>().unwrap_or(0);
            current_sum = match current_op {
                '+' => current_sum + num,
                '*' => current_sum * num,
                _ => current_sum,
            };
        }
    }

    total += current_sum;
    return total.to_string();
}
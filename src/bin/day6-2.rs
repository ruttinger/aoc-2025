use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    println!("{:?}", parse(&input));
}

fn parse(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let primops: Vec<&str> = lines
        .last()
        .expect("expected primitive operators")
        .split_whitespace()
        .collect();

    let grid_lines = &lines[..lines.len() - 1];

    let max_len = grid_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut columns: Vec<String> = vec![String::new(); max_len];

    for line in grid_lines.iter().rev() {
        for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                columns[i].push(ch);
            }
        }
    }

    let numbers: Vec<u64> = columns
        .iter()
        .filter_map(|s| {
            let rev: String = s.chars().rev().collect();
            rev.parse::<u64>().ok()
        })
        .collect();

    let mut sum = 0;

    let n_groups = primops.len();
    let total = numbers.len();

    let base = total / n_groups;
    let remainder = total % n_groups;

    let mut start = 0;

    for i in 0..n_groups {
        let size = if i < remainder { base + 1 } else { base };
        let end = start + size;

        let chunk = &numbers[start..end];

        if let Some(op) = primops.get(i) {
            if *op == "*" {
                let prod = chunk.iter().product::<u64>();
                sum += prod;
            } else if *op == "+" {
                sum += chunk.iter().sum::<u64>();
            }
        }

        start = end;
    }

    sum
}

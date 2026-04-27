use std::io;
use std::io::Read;
use std::cmp;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", parse(&input));
}

fn parse(input: &str) -> u64 {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    let mut active_cols: Vec<usize> = Vec::new();
    let mut split_count = 0;

    for line in lines.iter_mut() {
        if line.contains('S') {
            if let Some(col) = line.find('S') {
                active_cols.push(col);
            }
            continue;
        }

        let mut next_cols: Vec<usize> = Vec::new();

        for &col in &active_cols {
            if let Some(ch) = line.chars().nth(col) {
                match ch {
                    '.' => {
                        line.replace_range(col..=col, "|");
                        next_cols.push(col); // beam continues straight
                    }
                    '^' => {
                        if col > 0 {
                            line.replace_range((col-1)..=(col-1), "|");
                            next_cols.push(col - 1); // left branch
                        }
                        if col + 1 < line.len() {
                            line.replace_range((col+1)..=(col+1), "|");
                            next_cols.push(col + 1); // right branch
                        }
                        split_count += 1;
                    }
                    _ => {}
                }
            }
        }

        active_cols = next_cols;
    }

    split_count
}
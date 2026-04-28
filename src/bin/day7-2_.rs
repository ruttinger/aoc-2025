use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    println!("{}", parse(&input));
}

fn parse(input: &str) -> u64 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    
    let start_col = lines.iter()
        .find(|l| l.contains('S'))
        .and_then(|l| l.find('S'))
        .unwrap();

    let mut active: HashMap<usize, u64> = HashMap::new();
    active.insert(start_col, 1);

    for line in lines.iter().skip_while(|l| !l.contains('S')).skip(1) {
        let mut next: HashMap<usize, u64> = HashMap::new();

        for (col, count) in active {
            if let Some(ch) = line.chars().nth(col) {
                match ch {
                    '.' => { *next.entry(col).or_insert(0) += count; }
                    '^' => {
                        if col > 0 {
                            *next.entry(col - 1).or_insert(0) += count;
                        }
                        if col + 1 < line.len() {
                            *next.entry(col + 1).or_insert(0) += count;
                        }
                    }
                    _ => {}
                }
            }
        }

        active = next;
    }

    active.values().sum()
}
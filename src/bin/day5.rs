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
    let (first, second) = input.split_once("\n\n").unwrap();

    let mut ranges = Vec::new();
    for line in first.lines() {
        let (q, r) = line.split_once("-").unwrap();
        let k = q.parse::<u64>().unwrap();
        let n = r.parse::<u64>().unwrap();
        ranges.push((k, n))
    }

    let mut fresh = 0;

    for line in second.lines() {
        let x = line.parse::<u64>().unwrap();
        if ranges.iter().any(|&(k, n)| k <= x && x <= n) {
            fresh += 1;
        }
    }

    fresh
}

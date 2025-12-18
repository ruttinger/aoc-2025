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
    let mut ranges = Vec::new();

    let (first, _) = input.split_once("\n\n").unwrap();

    for line in first.lines() {
        let (q, r) = line.split_once("-").unwrap();
        let k = q.parse::<u64>().unwrap();
        let n = r.parse::<u64>().unwrap();
        ranges.push((k, n));
    }

    ranges.sort_unstable_by_key(|r| (r.0, r.1));

    let mut merged: Vec<(u64, u64)> = Vec::new();

    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 {
            current.1 = cmp::max(current.1, end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    merged
        .iter()
        .map(|(s, e)| e - s+1)
        .sum()

}

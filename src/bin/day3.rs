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
    let mut total_joltage = 0;

    for line in input.lines() {
        let joltage = interpret_12(line);
        println!("{}", joltage);
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

fn interpret12(line: &str) -> u64 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let n = digits.len();
    let k = 12;

    if n <= k {
        let mut joltage = digits.clone();
        joltage.resize(k, 0);
        return joltage.iter().fold(0, |acc, &d| acc * 10 + d as u64);
    }

    let mut joltage = digits[0..k].to_vec();
    let mut start = 0;

    for i in k..n {
        let remaining_digits = n - i;
        let positions_left = k - start;

        let mut p = start;
        while p < k {
            if digits[i] > joltage[p] && remaining_digits + (p + 1) >= k {
                joltage[p] = digits[i];
                start = p + 1;
                break;
            }
            p += 1;
        }
    }

    joltage.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

fn interpret_12(line: &str) -> u64 {
    let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
    let mut stack: Vec<u32> = Vec::with_capacity(12);

    let k = 12;
    let mut remaining = digits.len();

    for &d in &digits {
        // pop smaller digits if we can still fill to 12

        while let Some(&top) = stack.last() {
            if top < d && (stack.len() - 1 + remaining) >= k {
                stack.pop();
            } else {
                break;
            }
        }

        // push if we still need digits
        if stack.len() < k {
            stack.push(d);
        }

        remaining -= 1;
    }

    stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

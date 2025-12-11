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
    let mut paper_rolls = 0;

    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            let mut row: Vec<u32> = line
                .chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _   => panic!("unexpected char {}", c),
                })
                .collect();

            row.insert(0, 0);
            row.push(0);

            row
        })
        .collect();

    let width = grid[0].len();
    let pad = vec![0; width];
    grid.insert(0, pad.clone());
    grid.push(pad);

    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 1 {
                let rolls =
                    grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1] + grid[i][j - 1] +
                    grid[i][j + 1] + grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1];

                if rolls < 4 {
                    paper_rolls += 1;
                }
            }
        }
    }

    paper_rolls
}

use std::io;
use std::io::Read;
use rand::{Rng, RngExt};
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", parse(&input));
}
fn parse(input: &str) -> f64 {
    let points: Vec<(f64, f64, f64)> = input  // f64, not u64 — needed for euclidean
        .lines()
        .map(|line| {
            let nums: Vec<f64> = line
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (nums[0], nums[1], nums[2])
        })
        .collect();

    let mut rng = rand::rng();
    let mut d = f64::MAX;

    for _ in 0..points.len() {
        let i = rng.random_range(0..points.len());
        let j = rng.random_range(0..points.len());
        if i != j {
            let dist = euclidean(&points[i], &points[j]);
            d = d.min(dist);
        }
    }

    type GridCell = (i64, i64);

    let mut grid: HashMap<GridCell, Vec<usize>> = HashMap::new();

    for (idx, point) in points.iter().enumerate() {
        let cell = (
            (point.0 / d).floor() as i64,
            (point.1 / d).floor() as i64,
        );
        grid.entry(cell).or_default().push(idx);
    }

    let mut closest = f64::MAX;

    for (idx, point) in points.iter().enumerate() {
        let cx = (point.0 / d).floor() as i64;
        let cy = (point.1 / d).floor() as i64;

        // Check all 9 neighboring cells (Moore neighborhood)
        for dx in -1..=1 {
            for dy in -1..=1 {
                let neighbor_cell = (cx + dx, cy + dy);

                if let Some(neighbors) = grid.get(&neighbor_cell) {
                    for &other_idx in neighbors {
                        if other_idx != idx {
                            let dist = euclidean(point, &points[other_idx]);
                            closest = closest.min(dist);
                        }
                    }
                }
            }
        }
    }

    closest
}

fn euclidean(p1: &(f64, f64, f64), p2: &(f64, f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2) + (p1.2 - p2.2).powi(2)).sqrt()
}
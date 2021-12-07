use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: Vec<i32> = read_to_string("input.txt")?
        .split(",")
        .map(str::trim)
        .filter_map(|s| s.parse().ok())
        .collect();
    input.sort();

    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true));
    Ok(())
}

fn solve(input: &[i32], part2: bool) -> i32 {
    let p1_burn = |idx, pos| i32::abs(idx - pos);
    let p2_burn = |idx, pos| {
        let diff = i32::abs(idx - pos);
        diff * (diff + 1) / 2
    };

    let min = input[0];
    let max = input[input.len() - 1];
    (min..=max).map(|pos| {
        input.iter()
            .map(|curr| {
                if !part2 {
                    p1_burn(pos, curr)
                } else {
                    p2_burn(pos, curr)
                }
            })
            .fold(0, |acc, current| acc + current)
    }).min().unwrap()
}
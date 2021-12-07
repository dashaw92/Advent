use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: Vec<i32> = read_to_string("input.txt")?
        .split(",")
        .map(str::trim)
        .filter_map(|s| s.parse().ok())
        .collect();
    input.sort();
    let min = input[0];
    let max = input[input.len() - 1];

    let dist = (min..=max).map(|pos| {
        input.iter().fold(0, |acc, current| acc + i32::abs(pos - current))
    }).min().unwrap();
    println!("{} {} - min = {}", min, max, dist);

    Ok(())
}

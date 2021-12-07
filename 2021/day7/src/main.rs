use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: Vec<i32> = read_to_string("input.txt")?
        .split(",")
        .map(str::trim)
        .filter_map(|s| s.parse().ok())
        .collect();
    input.sort();

    println!("Part 1: {}", solve_p1(&input));
    println!("Part 2: {}", solve_p2(&input));
    Ok(())
}

fn solve_p1(input: &[i32]) -> i32 {
    let min = input[0];
    let max = input[input.len() - 1];
    (min..=max).map(|pos| {
        input.iter()
            .map(|curr| i32::abs(pos - curr))
            .fold(0, |acc, current| acc + current)
    }).min().unwrap()
}

//lol this doesn't finish with the debug build because of how inefficient it is
//TODO
fn solve_p2(input: &[i32]) -> i32 {
    let min = input[0];
    let max = input[input.len() - 1];
    (min..=max).map(|pos| {
        input.iter()
            .map(|curr| {
                let diff = i32::abs(pos - curr);
                let mut burn = 0;
                let mut counter = 1;
                for _ in 0..diff {
                    burn += counter;
                    counter += 1;
                }

                burn
            }).fold(0, |acc, current| acc + current)
    }).min().unwrap()
}
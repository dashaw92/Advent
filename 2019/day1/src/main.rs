use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let file = read_to_string("input.txt")?;
    let input: Vec<_> = file.lines().map(|line| line.as_ref()).collect();

    println!("Part 1: {}", Part1::solve(&input));
    println!("Part 2: {}", Part2::solve(&input));

    Ok(())
}

struct Part1;
struct Part2;

trait Solution<T, R> {
    fn solve(input: T) -> R;
}

impl Solution<&[&str], i32> for Part1 {
    fn solve(input: &[&str]) -> i32 {
        input
            .iter()
            .filter_map(|string| string.parse::<i32>().ok())
            .map(calculate_fuel)
            .sum()
    }
}

impl Solution<&[&str], i32> for Part2 {
    fn solve(input: &[&str]) -> i32 {
        let mut total = 0;
        let input = input.iter().filter_map(|line| line.parse::<i32>().ok()).collect::<Vec<_>>();

        for mut mass in input {
            while mass >= 0 {
                mass = calculate_fuel(mass);
                
                //don't allow negative values to taint the output
                if mass < 0 {
                    break;
                }

                total += mass;
            }
        }

        total
    }
}

fn calculate_fuel(mass: i32) -> i32 {
    f64::floor(mass as f64 / 3.) as i32 - 2
}
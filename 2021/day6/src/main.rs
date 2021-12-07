use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: Number of fish after 80 days: {}", solve(&input, 80));
    println!("Part 2: Number of fish after 256 days: {}", solve(&input, 256));
    Ok(())
}

fn solve(input: &str, num_days: usize) -> u128 {
    let fish: &mut [u128; 9] = &mut [0; 9];

    input.split(",")
        .map(str::trim)//I pulled my hair out over this for a solid 10 minutes (:
        .filter_map(|num| num.parse::<u128>().ok())
        .for_each(|num| fish[num as usize] += 1);

    for _ in 0..num_days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: &str = "3,4,3,1,2";

    #[test]
    fn small_p1() {
        assert_eq!(26, solve(PROVIDED, 18))
    }

    #[test]
    fn provided_p1() {
        assert_eq!(5934, solve(PROVIDED, 80))
    }

    #[test]
    fn provided_p2() {
        assert_eq!(26984457539, solve(PROVIDED, 256))
    }
}
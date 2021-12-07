use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: Number of fish after 80 days: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: &str) -> usize {
    let mut fish: Vec<Fish> = input.split(",")
        .map(Into::into)
        .collect();

    for _ in 0..80 {
        let new_fish: Vec<Fish> = fish.iter_mut().filter_map(|fish| fish.step()).collect();
        fish.extend(new_fish);
    }

    fish.len()
}

#[derive(Debug)]
struct Fish(isize);

impl Fish {
    fn step(&mut self) -> Option<Self> {
        self.0 -= 1;
        //Note: 0s are intentionally left!
        //-1 = new fish, not 0!
        if self.0 < 0 {
            self.0 = 6;
            return Some(Fish(8))
        }

        None
    }
}

impl From<&str> for Fish {
    fn from(num: &str) -> Self {
        let num = num.trim().parse().unwrap();
        Fish(num)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: &str = "3,4,3,1,2";

    #[test]
    fn provided_p1() {
        assert_eq!(5934, solve_p1(PROVIDED))
    }
}
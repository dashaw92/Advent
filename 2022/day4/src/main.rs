use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

type S = (usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<Job> = input.plumb();
    let p1 = input.iter().filter(|j| j.has_overlap()).count();
    (p1, 0)
}

#[derive(Debug)]
struct Job {
    fst: S,
    snd: S,
}

impl Job {
    fn has_overlap(&self) -> bool {
        let fst_contains_snd = self.fst.0 <= self.snd.0 && self.fst.1 >= self.snd.1;
        let snd_contains_fst = self.snd.0 <= self.fst.0 && self.snd.1 >= self.fst.1;

        fst_contains_snd || snd_contains_fst
    }
}

impl FromStr for Job {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (fst, snd) = s.split_once(',').unwrap();

        Ok(Job {
            fst: parse_sections(fst),
            snd: parse_sections(snd),
        })
    }
}

fn parse_sections(s: &str) -> S {
    let (s, e) = s.split_once('-').unwrap();
    let (s, e) = (s.parse().unwrap(), e.parse().unwrap());

    (s, e)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn provided_p1() {
        assert_eq!((2, 0), solve(PROVIDED));
    }
}

use aoc::Plumb;

use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

type M = HashSet<char>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<Sack> = input.plumb();
    let p1 = input.iter().map(union).sum();
    let p2 = input.chunks(3).map(find_badge).map(|c| priority(&c)).sum();

    (p1, p2)
}

struct Sack {
    all: String,
    fst: M,
    snd: M,
}

impl FromStr for Sack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() & 1 == 0);

        let mid = s.len() / 2;
        let (fst, snd) = s.split_at(mid);
        let (fst, snd) = (count_items(fst), count_items(snd));

        Ok(Sack {
            all: s.to_string(),
            fst,
            snd,
        })
    }
}

fn count_items<S: AsRef<str>>(s: S) -> M {
    let mut m = M::new();
    s.as_ref().chars().for_each(|c| {
        m.insert(c);
    });
    m
}

fn union(s: &Sack) -> usize {
    s.fst.intersection(&s.snd).map(priority).sum()
}

fn find_badge(ss: &[Sack]) -> char {
    let sets: Vec<M> = ss.iter().map(|sack| &sack.all).map(count_items).collect();

    *sets[0]
        .intersection(&sets[1])
        .find(|c| sets[2].contains(c))
        .unwrap()
}

fn priority(item: &char) -> usize {
    match item {
        'a'..='z' => ((*item as u8) - 96).into(),
        'A'..='Z' => ((*item as u8) - 38).into(),
        _ => unreachable!("Invalid char"),
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn provided_p1() {
        assert_eq!((157, 70), solve(PROVIDED));
    }
}

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

type P = (usize, usize);

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<Line> = read_to_string("input.txt")?
        .lines().map(Line::from).collect();

    println!("Part 1: Positions with 2 or more intersections: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(lines: &[Line]) -> usize {
    let mut map: HashMap<P, usize> = HashMap::new();
    lines.iter()
        .filter(|line| line.start.0 == line.end.0 || line.start.1 == line.end.1)
        .flat_map(|line| line.all_points().into_iter())
        .for_each(|p| *map.entry(p).or_insert(0) += 1);


    map.iter()
        .filter(|(_, &amount)| amount >= 2)
        .count()
}

struct Line {
    start: P,
    end: P,
}

impl Line {
    fn all_points(&self) -> Vec<P> {
        //p1: only vert or horiz

        //hmph
        if self.start.0 > self.end.0 {
            (self.end.0..=self.start.0).map(|pos| (pos, self.start.1)).collect()
        } else if self.start.0 < self.end.0 {
            (self.start.0..=self.end.0).map(|pos| (pos, self.start.1)).collect()
        } else if self.start.1 > self.end.1 {
            (self.end.1..=self.start.1).map(|pos| (self.start.0, pos)).collect()
        } else {
            (self.start.1..=self.end.1).map(|pos| (self.start.0, pos)).collect()
        }
    }
}

impl From<&str> for Line {
    fn from(line: &str) -> Self {
        let mut coords = line.split("->")
            .flat_map(|coords| coords.split(","))
            .map(str::trim)
            .filter_map(|coord| coord.parse().ok());

        let start = (coords.next().unwrap(), coords.next().unwrap());
        let end = (coords.next().unwrap(), coords.next().unwrap());
        Self { start, end }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: [&str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn provided_p1() {
        let lines: Vec<Line> = PROVIDED.iter()
            .map(|&s| s)
            .map(Line::from)
            .collect();

        assert_eq!(5, solve_p1(&lines))
    }
}
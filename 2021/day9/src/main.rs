use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn to_digits(input: impl AsRef<str>) -> Vec<usize> {
    input.as_ref()
        .lines()
        .flat_map(|s| s.split(""))
        .filter(|s| s.is_empty())
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

struct Heightmap {
    arr: Vec<usize>,
    dim: (usize, usize),
}
impl Heightmap {
    fn new(input: impl AsRef<str>) -> Self {
        let input: Vec<&str> = input.as_ref()
            .lines().collect();
        let width = input[0].len();
        let height = input.len();
        let arr = input.iter()
            .flat_map(|s| s.split(""))
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Self {
            arr,
            dim: (width, height),
        }
    }

    fn adjacent_to(&self, pos: (usize, usize)) -> (usize, Vec<usize>) {
        let (x, y) = pos;
        let (w, h) = self.dim;
        let idx = x + y * w;
        if idx >= self.arr.len() {
            return (0, Vec::with_capacity(0))
        }

        let mut adj = Vec::with_capacity(4);

        //above
        if y > 0 {
            adj.push(self.arr[idx - w])
        }

        //below
        if y < h - 1 {
            adj.push(self.arr[idx + w])
        }

        //left
        if x > 0 {
            adj.push(self.arr[idx - 1])
        }

        //right
        if x < w - 1 {
            adj.push(self.arr[idx + 1])
        }

        (self.arr[idx], adj)
    }
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    let map = Heightmap::new(&input);
    let mut heights = Vec::new();

    for y in 0..map.dim.1 {
        for x in 0..map.dim.0 {
            let (height, adj) = map.adjacent_to((x, y));
            if adj.iter().all(|&a| a > height) {
                heights.push(height)
            }
        }
    }

    heights.into_iter()
        .fold(0, |acc, height| acc + (height + 1))
}

fn solve_p2(input: impl  AsRef<str>) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn provided_p1() {
        assert_eq!(15, solve_p1(PROVIDED));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(0, solve_p2(PROVIDED));
    }
}

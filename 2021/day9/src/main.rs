use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::ops::Index;

type P = (usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    println!("Part 2: {}", solve_p2(&input));
    Ok(())
}

fn all_lower(height: usize, adj: &[usize]) -> bool {
    adj.into_iter().all(|&a| a > height)
}

struct Heightmap {
    arr: Vec<usize>,
    dim: P,
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

    fn adjacent_to(&self, pos: P) -> (usize, Vec<P>) {
        let (x, y) = pos;
        let (w, h) = self.dim;
        let idx = x + y * w;
        if idx >= self.arr.len() {
            return (0, Vec::with_capacity(0))
        }

        let mut adj = Vec::with_capacity(4);

        //above
        if y > 0 {
            adj.push((x, y - 1))
        }

        //below
        if y < h - 1 {
            adj.push((x, y + 1))
        }

        //left
        if x > 0 {
            adj.push((x - 1, y))
        }

        //right
        if x < w - 1 {
            adj.push((x + 1, y))
        }

        (self.arr[idx], adj)
    }
}

impl Index<(usize, usize)> for Heightmap {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.arr[index.0 + index.1 * self.dim.0]
    }
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    let map = Heightmap::new(&input);
    let mut heights = Vec::new();

    for y in 0..map.dim.1 {
        for x in 0..map.dim.0 {
            let (height, adj) = map.adjacent_to((x, y));
            let adj: Vec<usize> = adj.iter()
                .map(|&p| map[p])
                .collect();
            if all_lower(height, &adj) {
                heights.push(height)
            }
        }
    }

    heights.into_iter()
        .fold(0, |acc, height| acc + (height + 1))
}

fn solve_p2(input: impl AsRef<str>) -> usize {
    let map = Heightmap::new(&input);
    let mut basins = Vec::new();

    fn visit(map: &Heightmap, pos: P) -> HashSet<P> {
        let (h, adj) = map.adjacent_to(pos);
        let mut children: HashSet<P> = HashSet::new();
        children.insert(pos);

        for pos in adj {
            let pos_h = map[pos];
            if pos_h == 9 || pos_h <= h {
                continue
            }

            children.insert(pos);
            for child in visit(&map, pos) {
                children.insert(child);
            }
        }

        children
    }

    for y in 0..map.dim.1 {
        for x in 0..map.dim.0 {
            let pos = (x, y);
            let basin = visit(&map, pos);
            if basin.is_empty() {
                continue
            }
            // println!("{:?} -> {:?}", pos, basin);
            basins.push(basin.len());
        }
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));
    [basins[0], basins[1], basins[2]].into_iter().product()

    // let basin = visit(&map, (1, 0));
    // println!("{:?}", basin);
    // 0
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
        assert_eq!(1134, solve_p2(PROVIDED));
    }
}

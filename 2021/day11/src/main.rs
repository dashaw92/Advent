use std::error::Error;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let grid: Grid<10, 10, 100> = Grid::new(&input);
    println!("Part 1: {}", solve_p1(grid, 100));
    Ok(())
}

type P = (isize, isize);

#[derive(Debug)]
struct Grid<const W: usize, const H: usize, const LEN: usize> {
    grid: [isize; LEN],
    flashes: usize,
}

impl<const W: usize, const H: usize, const LEN: usize> Grid<W, H, LEN> {
    fn new(input: impl AsRef<str>) -> Self {
        let mut grid: [isize; LEN] = [0; LEN];
        input.as_ref()
            .split("")
            .map(str::trim)
            .filter_map(|s| s.parse().ok())
            .enumerate()
            .for_each(|(idx, num)| grid[idx] = num);

        Self {
            grid,
            flashes: 0,
        }
    }

    fn adj(&self, p: P) -> Vec<P> {
        let ps = [
            (p.0 - 1, p.1 - 1),
            (p.0 - 1, p.1),
            (p.0 - 1, p.1 + 1),
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1 - 1),
            (p.0 + 1, p.1),
            (p.0 + 1, p.1 + 1),
        ];

        let mut adj = Vec::with_capacity(ps.len());
        for p in ps {
            let idx = p.0 + p.1 * W as isize;
            if idx >= 0 && idx < self.grid.len() as isize {
                adj.push(p);
            }
        }
        adj
    }

    fn flash(&mut self, p: P) {
        self[p] = -1;
        self.flashes += 1;
        for a in self.adj(p) {
            match self[a] {
                9 => {
                    self.flash(a);
                    continue
                }
                -1 => continue,
                _ => self[a] += 1,
            }
        }
    }

    fn run_step(&mut self) {
        let pos: fn(usize) -> P = |idx| (idx as isize % W as isize, idx as isize / W as isize);

        (0..self.grid.len()).map(pos).for_each(|idx| {
            self[idx] += 1;
            if self[idx] > 9 {
                self.flash(idx);
            }
        });

        (0..self.grid.len()).map(pos).for_each(|idx| {
            if self[idx] == -1 {
                self[idx] = 0;
            }
        });
    }
}

impl<const W: usize, const H: usize, const LEN: usize> Index<P> for Grid<W, H, LEN> {
    type Output = isize;
    fn index(&self, index: P) -> &Self::Output {
        &self.grid[(index.0 + index.1 * W as isize) as usize]
    }
}
impl<const W: usize, const H: usize, const LEN: usize> IndexMut<P> for Grid<W, H, LEN> {
    fn index_mut(&mut self, index: P) -> &mut Self::Output {
        &mut self.grid[(index.0 + index.1 * W as isize) as usize]
    }
}

fn solve_p1<const W: usize, const H: usize, const LEN: usize>(mut grid: Grid<W, H, LEN>, steps: usize) -> usize {
    // let mut grid: Grid<10, 10> = Grid::new(input);
    (1..=steps).for_each(|_i| {
        println!("{} ->\n{:?}\n--------------------", _i, grid);
        grid.run_step();
    });
    grid.flashes
}

fn solve_p2(_input: impl  AsRef<str>) -> usize {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const SMALL: &str =
"11111
19991
19191
19991
11111";

    #[test]
    fn provided_small() {
        let grid: Grid<5, 5, 25> = Grid::new(SMALL);
        assert_eq!(9, solve_p1(grid, 3));
    }

    #[test]
    fn provided_p1() {
        let grid: Grid<10, 10, 100> = Grid::new(PROVIDED);
        assert_eq!(1656, solve_p1(grid, 100));
    }

    #[test]
    fn provided_p2() {
        assert_eq!(0, solve_p2(PROVIDED));
    }
}
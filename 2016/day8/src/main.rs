use aoc::Plumb;

use core::fmt;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let p1 = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: Code has been printed to screen, read it.");
    Ok(())
}

fn solve(input: impl AsRef<str>) -> usize {
    let mut g: Grid<51, 6> = Grid::new();
    let input: Vec<Cmd> = input.plumb();

    g.render(&input);
    let p1 = g.grid.iter().filter(|&b| *b).count();
    p1
}

enum Cmd {
    Rect { w: usize, h: usize },
    ShiftRow { y: usize, amount: usize },
    ShiftCol { x: usize, amount: usize },
}

struct Grid<const W: usize, const H: usize> {
    grid: Vec<bool>,
}

impl<const W: usize, const H: usize> fmt::Display for Grid<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..H {
            self.grid[y * W..(y + 1) * W - 1].iter().for_each(|b| {
                let _ = write!(
                    f,
                    "{}",
                    match b {
                        true => "█",
                        false => ".",
                    }
                );
            });
            writeln!(f)?;
        }

        writeln!(f, "{}x{}", W, H)
    }
}

impl<const W: usize, const H: usize> Grid<W, H> {
    fn new() -> Self {
        Self {
            grid: vec![false; W * H],
        }
    }

    fn render(&mut self, cmds: &[Cmd]) {
        for cmd in cmds {
            //indexes in the grid to shift
            //the amount to shift the indexes
            let (idxs, amount): (Vec<usize>, usize) = match cmd {
                Cmd::ShiftRow { y, amount } => (self.get_row(*y), *amount),
                Cmd::ShiftCol { x, amount } => (self.get_col(*x), *amount),

                //does not return in this iteration
                Cmd::Rect { w, h } => {
                    (0..*w).for_each(|x| {
                        (0..*h).for_each(|y| {
                            self.set(x, y, true);
                        });
                    });
                    continue;
                }
            };

            let mut vals: Vec<bool> = idxs.iter().map(|i| self.grid[*i]).collect();
            vals.as_mut_slice().rotate_right(amount % idxs.len());

            for (i, idx) in idxs.into_iter().enumerate() {
                self.grid[idx] = vals[i];
            }
        }

        println!("{self}");
    }

    fn set(&mut self, x: usize, y: usize, v: bool) {
        if x > W || y > H {
            return;
        }

        self.grid[y * W + x] = v;
    }

    fn get_row(&mut self, row: usize) -> Vec<usize> {
        let mut out = Vec::with_capacity(W);
        // &mut self.grid[row * self.width..(row + 1) * self.width - 1]
        (row * W..(row + 1) * W - 1).for_each(|idx| out.push(idx));
        out
    }

    fn get_col(&self, col: usize) -> Vec<usize> {
        let mut out = Vec::with_capacity(H);
        for idx in 0..H {
            out.push(col + idx * W);
        }

        out
    }
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("rect") {
            let (_, wxh) = s.split_once(' ').unwrap();
            let (w, h) = wxh
                .split_once('x')
                .map(|(w, h)| (w.parse().unwrap(), h.parse().unwrap()))
                .unwrap();

            return Ok(Cmd::Rect { w, h });
        }

        let (_, vals) = s.split_once('=').unwrap();
        let (a, b) = vals.split_once(" by ").unwrap();
        let pos = a.parse().unwrap();
        let amount = b.parse().unwrap();

        if s.contains("row") {
            Ok(Cmd::ShiftRow { y: pos, amount })
        } else {
            Ok(Cmd::ShiftCol { x: pos, amount })
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";

    #[test]
    fn provided_p1() {
        let mut g: Grid<8, 3> = Grid::new();
        let cmds = PROVIDED.plumb();
        g.render(&cmds);
    }
}

use aoc::grid::Grid;
use aoc::pos::Pos;

use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let map = Map::new(input);
    (0, 0)
}

#[derive(Debug)]
struct Map {
    nodes: HashMap<Node, Vec<Node>>,
    start: Pos<i32>,
    end: Pos<i32>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node {
    pos: Pos<i32>,
    height: u8,
}

impl Map {
    fn connections(&self, node: &Node) -> &[Node] {
        self.nodes.get(node).unwrap()
    }

    fn start(&self) -> &Node {
        self.node_at(self.start)
    }

    fn end(&self) -> &Node {
        self.node_at(self.end)
    }

    fn node_at(&self, pos: Pos<i32>) -> &Node {
        self.nodes.keys().find(|node| node.pos == pos).unwrap()
    }

    fn new(input: impl AsRef<str>) -> Self {
        let input = input.as_ref();
        let g_width = input.lines().next().map(str::len).unwrap();
        let g_height = input.lines().count();
        let input: Vec<char> = input.chars().filter(char::is_ascii_alphabetic).collect();
        let grid = Grid::from_iter(input.iter(), g_width, g_height);

        let ch_find = |input: &[char], target| -> Pos<i32> {
            input
                .iter()
                .enumerate()
                .find(|(_, &ch)| ch == target)
                .map(|(idx, _)| ((idx % g_width) as i32, (idx / g_width) as i32).into())
                .unwrap()
        };

        let start = ch_find(&input, 'S');
        let end = ch_find(&input, 'E');

        let nodes = input
            .iter()
            .enumerate()
            .map(|(idx, ch)| {
                (
                    ((idx % g_width) as i32, (idx / g_width) as i32).into(),
                    to_height(*ch),
                )
            })
            .map(|(pos, height)| {
                let node = Node { pos, height };
                let neighbors = [
                    pos - (0, 1).into(),
                    pos - (1, 0).into(),
                    pos + (0, 1).into(),
                    pos + (1, 0).into(),
                ];

                let connections: Vec<Node> = neighbors
                    .into_iter()
                    .filter(|n| {
                        *n.x() >= 0
                            && *n.x() < g_width as i32
                            && *n.y() >= 0
                            && *n.y() < g_height as i32
                    })
                    .map(|n| (n, grid[(*n.x() as usize, *n.y() as usize)]))
                    .map(|(n, other_height)| (n, to_height(*other_height)))
                    .filter(|(_, other_height)| {
                        *other_height + 1 == height || *other_height <= height
                    })
                    .map(|(n, other_height)| Node {
                        pos: n,
                        height: other_height,
                    })
                    .collect();

                (node, connections)
            })
            .collect();

        Self { nodes, start, end }
    }
}

fn to_height(ch: char) -> u8 {
    match ch {
        'E' => 25,
        'S' => 0,
        _ => ch as u8 - b'a',
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn provided_p1() {
        assert_eq!((0, 0), solve(PROVIDED));
    }
}

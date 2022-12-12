use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{grid::Grid, pos::Pos};

#[derive(Debug)]
pub struct Map {
    pub nodes: HashMap<Node, Vec<Node>>,
    pub start: Pos<i32>,
    pub end: Pos<i32>,
    pub size: (usize, usize),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Node {
    pub pos: Pos<i32>,
    pub height: u8,
}

impl Map {
    pub fn connections(&self, node: Pos<i32>) -> &[Node] {
        self.nodes.get(self.node_at(node)).unwrap()
    }

    pub fn start(&self) -> &Node {
        self.node_at(self.start)
    }

    pub fn end(&self) -> &Node {
        self.node_at(self.end)
    }

    pub fn node_at(&self, pos: Pos<i32>) -> &Node {
        self.nodes.keys().find(|node| node.pos == pos).unwrap()
    }

    pub fn new(input: impl AsRef<str>) -> Self {
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
                        *other_height == height + 1 || *other_height <= height
                    })
                    .map(|(n, other_height)| Node {
                        pos: n,
                        height: other_height,
                    })
                    .collect();

                (node, connections)
            })
            .collect();

        Self {
            nodes,
            start,
            end,
            size: (g_width, g_height),
        }
    }
}

fn to_height(ch: char) -> u8 {
    match ch {
        'E' => 25,
        'S' => 0,
        _ => ch as u8 - b'a',
    }
}

pub fn find_path(map: &Map) -> Vec<&Node> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let start = map.start();
    queue.push_back(start);
    visited.insert(start);

    let mut path = Vec::new();

    while let Some(current) = queue.pop_back() {
        path.push(current);
        if current == map.end() {
            break;
        }

        for node in map.connections(current.pos) {
            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);
            queue.push_back(node);
        }
    }

    path
}

pub fn print_path(map: &Map, path: &[&Node]) {
    let mut buf = vec!['·'; map.size.0 * map.size.1];

    for idx in 0..path.len() {
        let current = path[idx];

        let ch = if idx + 1 == path.len() {
            'Ω'
        } else if current == map.start() {
            'α'
        } else {
            let range = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let pct = ((idx as f64 / path.len() as f64) * range.len() as f64).floor() as usize;
            range[pct]
        };

        buf[*current.pos.y() as usize * map.size.0 + *current.pos.x() as usize] = ch;
    }

    for y in 0..map.size.1 {
        for x in 0..map.size.0 {
            print!("{} ", buf[y * map.size.0 + x]);
        }
        println!();
    }
}

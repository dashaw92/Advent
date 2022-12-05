use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

type M = HashMap<usize, VecDeque<char>>;

#[derive(Debug)]
pub(crate) struct Dock {
    stacks: M,
}

impl Dock {
    pub(crate) fn new(stacks: &[&str]) -> Self {
        let mut dock_stacks: M = stacks
            .last()
            .map(|s| s.split_ascii_whitespace())
            .unwrap()
            .flat_map(|num| num.parse().ok())
            .map(|num| (num, VecDeque::new()))
            .collect();

        for idx in (0..dock_stacks.len()).rev() {
            let line = stacks.get(idx).unwrap();

            for col in 0..dock_stacks.len() {
                let col_str = 4 * col + 1;
                let ch = line.chars().nth(col_str).unwrap();

                //TODO: Probably wrong
                if ch != ' ' {
                    dock_stacks.entry(col + 1).or_default().push_back(ch);
                }
            }
        }

        Self {
            stacks: dock_stacks,
        }
    }

    pub(crate) fn run_p1(&mut self, mvs: &[Move]) -> String {
        mvs.iter().for_each(|mv| {
            (0..mv.amt).for_each(|_| {
                let it = self.stacks.get_mut(&mv.src).unwrap().pop_back().unwrap();
                self.stacks.get_mut(&mv.dst).unwrap().push_back(it);
            });
        });

        self.get_p1_output()
    }

    pub(crate) fn run_p2(&mut self, mvs: &[Move]) -> String {
        mvs.iter().for_each(|mv| {
            let mut stack: Vec<char> = Vec::with_capacity(mv.amt);

            (0..mv.amt).for_each(|_| {
                let it = self.stacks.get_mut(&mv.src).unwrap().pop_back().unwrap();
                stack.push(it);
            });

            stack.reverse();
            stack.into_iter().for_each(|it| {
                self.stacks.get_mut(&mv.dst).unwrap().push_back(it);
            });
        });

        self.get_p1_output()
    }

    pub(crate) fn get_p1_output(&self) -> String {
        let mut buf = String::with_capacity(self.stacks.len());
        for i in 0..self.stacks.len() {
            let i = i + 1;
            let ch = *self.stacks.get(&i).unwrap().back().unwrap();
            buf.push(ch);
        }
        buf
    }
}

pub struct Move {
    amt: usize,
    src: usize,
    dst: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .split_ascii_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        Ok(Self {
            amt: nums[0],
            src: nums[1],
            dst: nums[2],
        })
    }
}

use core::fmt;
use std::{num::ParseIntError, str::FromStr};

pub use Op::*;
pub use Var::*;

//I'm getting sick of having to change types everywhere
//This alleviates that pain
pub type Int = u64;

pub fn parse_monkeys(input: impl AsRef<str>, split: &str) -> Vec<Monkey> {
    input
        .as_ref()
        .split(split)
        // .inspect(|monkey| println!("{}", monkey.escape_debug()))
        .filter_map(|monkey| monkey.parse().ok())
        .collect()
}

pub struct Monkey {
    pub id: usize,
    pub items: Vec<Int>,
    pub op: Op,
    pub pred: Predicate,
    pub inspected: usize,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split('\n').collect();

        let id = parse_tup1_num(lines[0].split_once(' '));

        let items = lines[1]
            .split_once(':')
            .map(|(_, items)| items.trim())
            .iter()
            .flat_map(|items| items.split(", "))
            .filter_map(|item| item.parse().ok())
            .collect();

        let op = lines[2].parse().unwrap();
        let pred = lines[3..=5].into();

        Ok(Self {
            id,
            items,
            op,
            pred,
            inspected: 0,
        })
    }
}

pub struct Predicate {
    pub modulo: Int,
    pub pass: usize,
    pub fail: usize,
}

impl Predicate {
    pub fn test(&self, item: Int) -> usize {
        if item % self.modulo == 0 {
            self.pass
        } else {
            self.fail
        }
    }
}

impl From<&[&str]> for Predicate {
    fn from(lines: &[&str]) -> Self {
        let modulo = parse_tup1_num(lines[0].split_once("by"));

        let pass = parse_tup1_num(lines[1].split_once("monkey"));
        let fail = parse_tup1_num(lines[2].split_once("monkey"));

        Self { modulo, pass, fail }
    }
}

pub enum Op {
    Mul(Var),
    Add(Var),
}

impl Op {
    pub fn apply(&self, item: Int) -> Int {
        match self {
            Mul(Prev) => item * item,
            Mul(Const(x)) => item * x,
            Add(Prev) => item + item,
            Add(Const(x)) => item + x,
        }
    }
}

//Operation is always "old * <var>"
pub enum Var {
    Const(Int),
    Prev, //old * old
          //      ^^^
}

impl FromStr for Op {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once('=').unwrap().1.trim();
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();

        match (tokens[1], tokens[2]) {
            ("+", var) => {
                if var == "old" {
                    Ok(Add(Prev))
                } else {
                    Ok(Add(Const(var.parse().unwrap())))
                }
            }
            ("*", var) => {
                if var == "old" {
                    Ok(Mul(Prev))
                } else {
                    Ok(Mul(Const(var.parse().unwrap())))
                }
            }
            _ => unreachable!("Bad input! Tokens provided: {tokens:#?}\nRaw: {s}"),
        }
    }
}

fn parse_tup1_num<T: FromStr>(opt: Option<(&str, &str)>) -> T {
    opt.map(|(_, x)| x.trim())
        .map(|x| x.replace(':', ""))
        .and_then(|num| num.parse::<T>().ok())
        .unwrap()
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Monkey {}:", self.id)?;
        writeln!(f, "  Starting items: {:?}", &self.items)?;
        writeln!(
            f,
            "  Operation: new = old {}",
            match self.op {
                Mul(Prev) => "* old".into(),
                Mul(Const(x)) => format!("* {}", x),
                Add(Prev) => "+ old".into(),
                Add(Const(x)) => format!("+ {}", x),
            }
        )?;
        writeln!(f, "  Test: divisible by {}", self.pred.modulo)?;
        writeln!(f, "    If true: throw to monkey {}", self.pred.pass)?;
        writeln!(f, "    If false: throw to monkey {}", self.pred.fail)
    }
}

use std::str::FromStr;

pub struct Pair {
    left: List,
    right: List,
}

pub enum List {
    Many(Vec<List>),
    One(i32),
}

impl FromStr for List {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = List::Many(vec![]);
        let mut current = &mut root;

        let chars: Vec<char> = s.chars().collect();
        let mut idx = 0;
        while idx < chars.len() {}
        Ok(root)
    }
}

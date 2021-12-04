use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub(crate) struct BingoCard {
    board: Vec<Slot>,
    dims: (usize, usize),
}

impl BingoCard {
    pub fn new(numbers: &[u8]) -> Self {
        let board = numbers.iter().map(Into::into).collect();

        Self {
            board,
            dims: (5, 5),
        }
    }

    pub fn slot(&self, x: usize, y: usize) -> Option<&Slot> {
        if x > self.dims.0 || y > self.dims.1 {
            return None
        }

        Some(&self.board[x + y * self.dims.0])
    }

    pub fn row(&self, y: usize) -> Option<&[Slot]> {
        if y > self.dims.1 {
            return None
        }

        let idx = y * self.dims.0;
        Some(&self.board[idx..idx + self.dims.0])
    }

    pub fn column(&self, x: usize) -> Option<Vec<Slot>> {
        if x > self.dims.0 {
            return None
        }

        let mut slots = Vec::new();
        let mut idx = x;
        while idx < self.dims.0 * self.dims.1 {
            slots.push(self.board[idx].clone());
            idx += self.dims.0;
        }

        Some(slots)
    }

    pub fn is_winner(&self) -> bool {
        for idx in 0..self.dims.0 {
            let row = self.row(idx).unwrap();
            if row.iter().all(Slot::is_marked) {
                return true
            }

            let column = self.column(idx).unwrap();
            if column.iter().all(Slot::is_marked) {
                return true
            }
        }

        false
    }

    pub fn mark(&mut self, num: u8) {
        for idx in 0..self.dims.0 * self.dims.1 {
            match self.board[idx] {
                Slot::Unmarked(val) if val == num => {
                    self.board[idx] = Slot::Marked(val);
                },
                _ => (),
            }
        }
    }

    pub fn score(&self, last_draw: impl Into<usize>) -> usize {
        self.board
            .iter()
            .filter(|slot| !slot.is_marked())
            .map(|slot| **slot as usize)
            .sum::<usize>() * last_draw.into()
    }
}

impl Display for BingoCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        let mut line = String::new();
        for idx in 0..self.dims.0 * self.dims.1 {
            line.push_str(&format!("{:>3}", self.slot(idx % self.dims.0, idx / self.dims.1).unwrap().to_string()));
            line += " ";

            if idx % self.dims.0 == self.dims.0 - 1 {
                buffer.push_str(&line);
                line.clear();
                buffer += "\n";
            }
        }

        write!(f, "{}", buffer)
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Slot {
    Marked(u8),
    Unmarked(u8),
}
impl Slot {
    pub fn is_marked(&self) -> bool {
        if let Self::Marked(_) = self {
            true
        } else {
            false
        }
    }
}
impl From<&u8> for Slot {
    fn from(val: &u8) -> Self {
        Self::Unmarked(*val)
    }
}
impl Display for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Marked(val) => format!("*{}", val),
            Self::Unmarked(val) => format!("{}", val),
        })
    }
}
impl Deref for Slot {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Marked(val) => &val,
            Self::Unmarked(val) => &val,
        }
    }
}
impl DerefMut for Slot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Marked(val) => val,
            Self::Unmarked(val) => val,
        }
    }
}
use core::fmt;
use std::{
    fmt::Display,
    ops::{Deref, DerefMut, Index, IndexMut},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    inner: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            inner: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn from_iter(iter: impl Iterator<Item = T>, width: usize, height: usize) -> Self {
        let inner: Vec<T> = iter.collect();

        assert_eq!(
            inner.len(),
            width * height,
            "Grid size does not match `{width}x{height}` (should be {}, actual is {})!",
            width * height,
            inner.len()
        );

        Self {
            inner,
            width,
            height,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.inner[y * self.width + x])
        }
    }

    pub fn index_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.inner[y * self.width + x])
        }
    }

    pub fn to_xy(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;

        (x, y)
    }

    pub fn to_idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.width + x
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        self.index(x, y)
            .unwrap_or_else(|| panic!("Position ({}x{}) out of bounds", x, y))
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        self.index_mut(x, y)
            .unwrap_or_else(|| panic!("Position ({}, {}) out of bounds", x, y))
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let elem = &self[(x, y)];
                write!(f, "{elem} ")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct Pos<T: Copy>(T, T);

impl<T> Pos<T>
where
    T: Copy,
{
    pub fn new((x, y): (T, T)) -> Self {
        Pos(x, y)
    }

    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }
}

impl<T> Pos<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    fn add(&self, other: &Pos<T>) -> Pos<T> {
        Pos(self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: &Pos<T>) -> Pos<T> {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> From<(T, T)> for Pos<T>
where
    T: Copy,
{
    fn from(xy: (T, T)) -> Self {
        Pos::new(xy)
    }
}

impl<T> Sub for Pos<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Pos<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::sub(&self, &rhs)
    }
}

impl<T> Add for Pos<T>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Pos<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::add(&self, &rhs)
    }
}

impl<T> AddAssign for Pos<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T> SubAssign for Pos<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T> fmt::Display for Pos<T>
where
    T: std::fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pos").field(&self.0).field(&self.1).finish()
    }
}

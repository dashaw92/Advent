struct Pos<T>(T, T);

impl<T> Pos<T> {
    fn new((x, y): (T, T)) -> Self {
        Pos(x, y)
    }

    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }
}

impl<T> Pos<T>
where
    T: Add<Rhs = T> + Sub<Rhs = T>,
{
    fn add(&self, other: &Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> From<(T, T)> for Pos {
    fn from(xy: (T, T)) -> Self {
        Pos::new(xy)
    }
}

impl<T> ops::Sub for Pos<T>
where
    T: Add<Rhs = T> + Sub<Rhs = T>,
{
    type Output = Pos;

    fn sub(self, rhs: Self) -> Self::Output {
        Pos::add(&self, &-rhs)
    }
}

impl ops::Add for Pos
where
    T: Add<Rhs = T> + Sub<Rhs = T>,
{
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos::add(&self, &rhs)
    }
}

impl Neg for Pos
where
    T: Neg<T>,
{
    type Output = Pos;

    fn neg(self) -> Self::Output {
        Pos(-self.x(), -self.y())
    }
}

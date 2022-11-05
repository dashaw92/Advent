use std::str::FromStr;

pub trait Plumb<T> {
    fn plumb(&self) -> T;
}

impl<T, U> Plumb<Vec<T>> for U
where
    T: FromStr,
    U: AsRef<str>,
{
    fn plumb(&self) -> Vec<T> {
        self.as_ref()
            .lines()
            .filter_map(|s| s.parse::<T>().ok())
            .collect()
    }
}

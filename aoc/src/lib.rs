use std::fs::read_to_string;
use std::path::Path;

//I found this idea on https://github.com/AlexAegis/advent-of-code
pub trait Solution<T, R> {
    fn solve(input: T) -> R;
}

pub fn read_input<P: AsRef<Path>>(file: P) -> String {
    match read_to_string(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e),
    }
}
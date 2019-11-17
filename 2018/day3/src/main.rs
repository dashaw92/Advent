use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("The total overlap is {} square inches", part1(INPUT));
}

fn part1(input: &str) -> i32 {
    let pattern = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let mut total = 0;

    let iter = input.lines().map(|line| {
        let matches = pattern.captures(line).unwrap();
        let id = matches.get(1).unwrap().as_str().parse().unwrap();
        let x = matches.get(2).unwrap().as_str().parse().unwrap();
        let y = matches.get(3).unwrap().as_str().parse().unwrap();
        let w = matches.get(4).unwrap().as_str().parse().unwrap();
        let h = matches.get(5).unwrap().as_str().parse().unwrap();

        Rect::new(id, x, y, w, h)
    });

    //do some really cool stuff that for some reason, I can't figure out at all
    //it's just rectangles, yet it makes no sense to me...

    total
}

struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn new(id: i32, x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { id, x, y, w, h }
    }

    fn overlaps(&self, other: &Rect) -> bool {
        self.x >= other.x && //stop
        self.y >= other.y && //it
        other.x <= self.x + self.w && //cargo
        other.y <= self.y + self.h //fmt
    }

    fn sq_inch_overlap(&self, other: &Rect) -> i32 {
        if !self.overlaps(other) {
            return 0;
        }

        //some really cool code that calculates the overlap area

        unimplemented!()
    }
}

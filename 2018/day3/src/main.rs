use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let solution = solve(INPUT);
    println!("The total overlap is {} sq. inches", solution.0);
    println!("Claim ID {} did not overlap any other claims.", solution.1);
}

//format:
//#<id> @ <x1>,<y1>: <width>x<height>
fn collect_rects(input: &str) -> HashMap<i32, Rect> {
    // "x,y:" -> (x, y)
    // "AxB" -> (A, B)
    fn parse_coords(coords: &str, separator: char, delimeter: Option<char>) -> (i32, i32) {
        //needed because strip_suffix returns None if the provided pattern isn't found
        //also, working with `stripped` as an Option makes the code much more fluent
        let stripped = match delimeter {
            Some(delim) => coords.strip_suffix(delim),
            None => Some(coords),
        };

        stripped
            //split on the provided separator
            .map(|coords| coords.split(separator))
            //collect into a tuple
            .map(|mut coords| (coords.next().unwrap(), coords.next().unwrap()))
            //parse the tuple as i32s
            .map(|coords| (coords.0.parse().unwrap(), coords.1.parse().unwrap()))
            .unwrap()
    }

    let mut rects = HashMap::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let id = split
            .next()
            .and_then(|st| st.strip_prefix('#'))
            .and_then(|id| id.parse::<i32>().ok())
            .unwrap();

        let (x1, y1) = parse_coords(split.nth(1).unwrap(), ',', Some(':'));
        let (w, h) = parse_coords(split.next().unwrap(), 'x', None);

        rects.insert(
            id,
            Rect {
                x1,
                y1,
                x2: w + x1,
                y2: h + y1,
            },
        );
    }
    rects
}

fn solve(input: &str) -> (i32, i32) {
    let rects = collect_rects(input);
    let mut board = vec![vec![Vec::new(); 1000]; 1000];

    let mut candidates_p2: HashSet<i32> = rects.keys().copied().collect();
    for (id, r) in rects.iter() {
        for x in r.x1..r.x2 {
            for y in r.y1..r.y2 {
                let coord = &mut board[y as usize][x as usize];
                coord.push(id);
                if coord.len() > 1 {
                    coord.iter().for_each(|&rect_id| {
                        candidates_p2.remove(rect_id);
                    });
                }
            }
        }
    }

    let overlap = board
        .iter()
        .flatten()
        .filter(|&coord| coord.len() > 1)
        .fold(0, |acc, _| acc + 1);

    (overlap, *candidates_p2.iter().next().unwrap())
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &'static str = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;

    #[test]
    fn provided() {
        assert_eq!((4, 3), solve(PROVIDED));
    }
}

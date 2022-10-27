use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("The total overlap is {} sq. inches", part1(INPUT));
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

fn part1(input: &str) -> i32 {
    let rects = collect_rects(input);
    let mut board = vec![vec![0; 1000]; 1000];

    for r in rects.values() {
        for x in r.x1..r.x2 {
            for y in r.y1..r.y2 {
                board[y as usize][x as usize] += 1;
            }
        }
    }

    // dbg_board(&board);

    board
        .iter()
        .flatten()
        .filter(|&&coord| coord > 1)
        .fold(0, |acc, _| acc + 1)
}

// fn dbg_board(board: &[Vec<i32>]) {
//     for line in board {
//         println!("{line:?}");
//     }
// }

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
        assert_eq!(4, part1(PROVIDED));
    }
}

use std::error::Error;
use std::fs::read_to_string;
use aoc::bingo::BingoCard;

fn main() -> Result<(), Box<dyn Error>>{
    let input: Vec<String> = read_to_string("input.txt")?.lines().map(ToOwned::to_owned).collect();

    let solutions = solve(&input);
    println!("Part 1: Score of first winning card: {}", solutions.0);
    println!("Part 2: Score of last winning card: {}", solutions.1);
    Ok(())
}

fn solve(input: &[String]) -> (usize, usize) {
    let numbers: Vec<u8> = input[0].split(",")
        .filter_map(|num| num.parse().ok())
        .collect();

    let mut cards: Vec<BingoCard> = input[2..].split(|x| x.is_empty())
        .map(|slots| slots.join(" "))
        .map(|slots| {
            slots.split_ascii_whitespace()
                .filter_map(|slot| slot.parse().ok())
                .collect::<Vec<u8>>()
        })
        .map(|board| BingoCard::new(&board))
        .collect();

    let mut winners = Vec::with_capacity(cards.len());
    let mut first_score = 0;
    let mut last_num = 0;
    let total = cards.len();

    'outer: for num in numbers {
        for (idx, card) in cards.iter_mut().enumerate() {
            card.mark(num);
            if card.is_winner() && !winners.contains(&idx) {
                if winners.is_empty() {
                    first_score = card.score(num);
                }

                winners.push(idx);
                if winners.len() == total {
                    break 'outer
                }
            }

            last_num = num;
        }
    }

    let last_score = winners.last()
        .map(|&idx| cards[idx].score(last_num))
        .unwrap_or(0);
    (first_score, last_score)
}

#[cfg(test)]
mod test {
    const INPUT: &'static str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    use super::*;

    #[test]
    fn provided() {
        let lines: Vec<String> = INPUT.lines().map(ToOwned::to_owned).collect();
        assert_eq!((4512, 1924), solve(&lines));
    }
}
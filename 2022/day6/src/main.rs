use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let chars: Vec<char> = input.as_ref().chars().collect();

    let p1 = find(&chars, 4);
    let p2 = find(&chars, 14);
    (p1, p2)
}

fn find(chars: &[char], length: usize) -> usize {
    chars
        .windows(length)
        .enumerate()
        .find(|(_, seq)| is_marker(seq, length))
        .map(|(i, _)| i + length)
        .unwrap()
}

fn is_marker(seq: &[char], length: usize) -> bool {
    let mut bits: u32 = 0;
    seq.iter()
        .map(|&ch| (ch as u8) - b'a')
        .for_each(|bit| bits |= 1 << bit);

    println!("{seq:?} {bits:032b}");
    bits.count_ones() == length as u32
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: [(&str, (usize, usize)); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", (7, 19)),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", (5, 23)),
        ("nppdvjthqldpwncqszvftbrmjlhg", (6, 23)),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", (10, 29)),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", (11, 26)),
    ];

    #[test]
    fn provided_p1() {
        PROVIDED
            .into_iter()
            .for_each(|test| assert_eq!(solve(test.0), test.1));
    }
}

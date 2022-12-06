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

    let p1 = chars
        .windows(4)
        .enumerate()
        .find(|(_, seq)| is_packet_marker(seq))
        .map(|(i, _)| i + 4)
        .unwrap();
    (p1, 0)
}

fn is_packet_marker(seq: &[char]) -> bool {
    let mut bits: u32 = 0;
    seq.iter()
        .map(|&ch| (ch as u8) - b'a')
        .for_each(|bit| bits |= 1 << bit);

    bits.count_ones() == 4
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: [(&str, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn provided_p1() {
        PROVIDED
            .into_iter()
            .for_each(|test| assert_eq!(solve(test.0).0, test.1));
    }
}

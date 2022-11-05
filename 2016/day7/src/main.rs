use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, _p2) = solve(&input);
    println!("Part 1: {}", p1);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<IP> = input.plumb();
    let p1 = input.iter().filter(|ip| ip.is_valid()).count();
    (p1, 0)
}

//abba[mnop]qrst
//^    ^    ^
//-----|---------outside
//     inside
#[derive(Debug)]
struct IP {
    outside: Vec<String>,
    inside: Vec<String>,
}

impl FromStr for IP {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inside = Vec::new();
        let mut outside = Vec::new();

        let mut buf = String::new();
        let mut in_sq = false;
        for c in s.chars() {
            match c {
                '[' => {
                    outside.push(buf);
                    buf = String::new();
                    in_sq = true;
                    continue;
                }
                ']' => {
                    inside.push(buf);
                    buf = String::new();
                    in_sq = false;
                    continue;
                }
                _ => {}
            }

            buf.push(c);
        }

        if in_sq {
            inside.push(buf);
        } else {
            outside.push(buf);
        }

        Ok(Self { outside, inside })
    }
}

impl IP {
    fn is_valid(&self) -> bool {
        let has_pair_outside = pair_check(&self.outside);
        let has_pair_inside = pair_check(&self.inside);

        has_pair_outside && !has_pair_inside
    }
}

fn pair_check(v: &[String]) -> bool {
    v.iter()
        .map(String::as_bytes)
        .flat_map(|b| b.windows(4))
        .any(is_pair)
}

fn is_pair(sl: &[u8]) -> bool {
    sl.len() >= 4 && //first a length check
    sl[0] == sl[3] && sl[1] == sl[2] && //xyyx
    sl[0] != sl[1] //disallow xxxx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ip: IP = "abba[mnop]qrst".parse().unwrap();
        assert!(ip.is_valid());

        let ip: IP = "abcd[bddb]xyyx".parse().unwrap();
        assert!(!ip.is_valid());
    }
}

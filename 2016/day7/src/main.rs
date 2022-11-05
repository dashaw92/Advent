use aoc::Plumb;

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>) -> (usize, usize) {
    let input: Vec<IP> = input.plumb();
    let p1 = input.iter().filter(|ip| ip.is_abba()).count();
    let p2 = input.iter().filter(|ip| ip.is_ssl()).count();
    (p1, p2)
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
    fn is_abba(&self) -> bool {
        let has_pair_outside = pair_check(&self.outside, 4, is_abba);
        let has_pair_inside = pair_check(&self.inside, 4, is_abba);

        has_pair_outside && !has_pair_inside
    }

    fn is_ssl(&self) -> bool {
        //has ABA in outside
        //has BAB in inside
        for pair in self.outside.iter().flat_map(find_aba) {
            if self.inside.iter().any(|word| has_bab(word, pair)) {
                return true;
            }
        }

        false
    }
}

fn pair_check<F>(v: &[String], w_size: usize, fun: F) -> bool
where
    F: Fn(&[u8]) -> bool,
{
    v.iter()
        .map(String::as_bytes)
        .flat_map(|b| b.windows(w_size))
        .any(fun)
}

fn is_abba(sl: &[u8]) -> bool {
    sl.len() >= 4 && //first a length check
    sl[0] == sl[3] && sl[1] == sl[2] && //xyyx
    sl[0] != sl[1] //disallow xxxx
}

fn find_aba(sl: &String) -> impl Iterator<Item = (u8, u8)> + '_ {
    sl.as_bytes()
        .windows(3)
        .filter(|sl| sl[0] == sl[2] && sl[0] != sl[1])
        .map(|sl| (sl[0], sl[1]))
}

fn has_bab(sl: &String, (a, b): (u8, u8)) -> bool {
    sl.as_bytes()
        .windows(3)
        .any(|sl| sl[0] == b && sl[0] == sl[2] && sl[1] == a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abba() {
        let ip: IP = "abba[mnop]qrst".parse().unwrap();
        assert!(ip.is_abba());

        let ip: IP = "abcd[bddb]xyyx".parse().unwrap();
        assert!(!ip.is_abba());
    }

    #[test]
    fn test_ssl() {
        // let ip: IP = "aba[bab]xyz".parse().unwrap();
        // assert!(ip.is_ssl());

        // let ip: IP = "aaa[kek]eke".parse().unwrap();
        // assert!(ip.is_ssl());

        let ip: IP = "zazbz[bzb]cdb".parse().unwrap();
        assert!(ip.is_ssl());

        // let ip: IP = "xyx[xyx]xyx".parse().unwrap();
        // assert!(!ip.is_ssl());
    }
}

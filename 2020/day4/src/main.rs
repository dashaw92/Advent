use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<String> = read_to_string("input.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    let ans = solve(&input);
    println!("Part 1: {} valid passports", ans.0);
    println!("Part 2: {} valid passports", ans.1);
    Ok(())
}

fn solve(input: &[impl AsRef<str>]) -> (usize, usize) {
    let input: Vec<&str> = input.iter().map(AsRef::as_ref).collect();
    let mut passports = Vec::new();

    for passport in input.split(|line| line.is_empty()) {
        let map: HashMap<String, String> = passport.iter().flat_map(|s| s.split_ascii_whitespace()).map(|pair| {
            let pair = pair.to_lowercase();
            let mut kv = pair.split(":");
            (kv.next().unwrap().to_owned(), kv.next().unwrap().to_owned())
        }).collect();

        let mut pass = P::new(map);
        passports.push(pass);
    }

    let valid_p1 = passports.iter()
        .filter(|&p| p.has_all_fields())
        .count();

    let valid_p2 = passports.iter()
        .filter(|&p| p.is_valid())
        .count();
    (valid_p1, valid_p2)
}

#[cfg(test)]
mod test {
    use super::*;

    const PROVIDED: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:Amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn provided() {
        let input: Vec<&str> = PROVIDED.lines().collect();
        let ans = solve(&input);
        assert_eq!(2, ans.0);
    }
}
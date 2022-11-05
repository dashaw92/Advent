use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let mut rooms: Vec<Room> = input
        .lines()
        .filter_map(|s| s.parse().ok())
        .filter(Room::is_valid)
        .collect();

    let sum_sectors = rooms.iter().fold(0, |acc, r| acc + r.sector);
    println!("P1: {sum_sectors}");

    rooms.iter_mut().for_each(Room::shift_name);
    let north_pole_storage = rooms.iter().find(|r| r.name.contains("northpole")).unwrap();
    println!("P2: {}", north_pole_storage.sector);
    Ok(())
}

#[derive(Debug)]
struct Room {
    name: String,
    sector: usize,
    checksum: [char; 5],
}

impl FromStr for Room {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name: String = s
            .split('-')
            .filter(|p| !p.contains('['))
            .fold(String::new(), |acc, w| format!("{acc}-{w}"));
        let name = name.strip_prefix('-').map(ToOwned::to_owned).unwrap();

        let (sector, checksum) = s.rsplit_once('-').unwrap().1.split_once('[').unwrap();
        let sector = sector.parse().unwrap();
        let checksum_s = checksum.strip_suffix(']').unwrap();

        let mut checksum = ['\0'; 5];
        //according to clippy, this is more idiomatic rust than just a simple for i in 0..5 loop :raised_eyebrow:
        (0..5).for_each(|i| {
            checksum[i] = checksum_s.chars().nth(i).unwrap();
        });

        Ok(Self {
            name,
            sector,
            checksum,
        })
    }
}

impl Room {
    fn is_valid(&self) -> bool {
        //helper for inserting and updating a frequency map in Vec form.
        //done this way so I can sort the keys by frequency easily to get
        //the top 5 for validating the Room's checksum.
        fn update(col: &mut Vec<(char, u32)>, c: char) {
            for pair in col.iter_mut() {
                if pair.0 == c {
                    pair.1 += 1;
                    return;
                }
            }

            col.push((c, 1));
        }

        let mut freq = Vec::new();
        self.name
            .chars()
            .filter(|&c| c != '-')
            .for_each(|c| update(&mut freq, c));
        freq.sort_by(|a, b| {
            //Check if the frequency for these chars are the same.
            //If so, sort by alphabetic ordering
            //
            //Otherwise, sort by the frequencies in descending order, so more common
            //chars appear earlier in the array
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        freq.into_iter()
            .map(|(a, _)| a)
            .take(5)
            .zip(self.checksum.iter())
            .all(|(c1, c2)| c1 == *c2)
    }

    fn shift_name(&mut self) {
        let name: String = self.name.chars().map(|c| rotate(c, self.sector)).collect();
        self.name = name;
    }
}

fn rotate(c: char, amt: usize) -> char {
    if c == '-' {
        return ' ';
    }
    let idx = c as u8 - b'a';
    let shifted = idx as usize + amt;
    let modulo = shifted % 26;
    let offset = modulo as u8 + b'a';
    offset as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let rooms: &[(&str, bool)] = &[
            ("aaaaa-bbb-z-y-x-123[abxyz]", true),
            ("a-b-c-d-e-f-g-h-987[abcde]", true),
            ("not-a-real-room-404[oarel]", true),
            ("totally-real-room-200[decoy]", false),
        ];

        for pair in rooms {
            let room: Room = pair.0.parse().unwrap();
            assert_eq!(room.is_valid(), pair.1);
        }
    }

    #[test]
    fn test_rotate() {
        let mut r = Room::from_str("qzmt-zixmtkozy-ivhz-343[abcde]").unwrap();
        r.shift_name();
        println!("{}", r.name);
        assert_eq!('b', rotate('a', 27));
    }
}

use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let sum_sectors = input
        .lines()
        .filter_map(|s| s.parse().ok())
        .filter(Room::is_valid)
        .fold(0, |acc, room| acc + room.sector);
    println!("P1: {sum_sectors}");
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
        let name: String = s.split('-').filter(|p| !p.contains('[')).collect();
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
        self.name.chars().for_each(|c| update(&mut freq, c));
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
            println!("{} {room:?}", pair.0);
            assert_eq!(room.is_valid(), pair.1);
        }
    }
}

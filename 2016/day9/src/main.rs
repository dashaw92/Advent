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
    let mut buf = Vec::with_capacity(input.as_ref().len());
    let input = input.as_ref();
    let chars: Vec<char> = input.chars().collect();

    // Simple strategy: Iterate over every char in the input. The only special case that needs to be
    // considered is the start of a marker. Note that the input can contain '(' as a literal character,
    // but the way markers are consumed from the iteration is greedy, so no extra consideration is needed
    // in this case.
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            //This is the start of a marker
            '(' => {
                //We need to find the end of the marker because markers
                //can have variable widths based on the digits used,
                //i.e "(10x50)" vs "(1x2)". Just mark the current position
                //and increase it until we find the first ')'. This will
                //give us the range in the string from which to extract
                //the complete marker.
                let mut end = i;
                while end < chars.len() && chars[end] != ')' {
                    end += 1;
                }

                //Ensure we found a marker. This will panic if the marker does not
                //finish. Needs testing.
                match &chars[i..=end] {
                    //x: The whole marker with '(' and ')'
                    //inner: Everything between the parens
                    x @ &['(', ref inner @ .., ')'] => {
                        //Build the string for parsing into ints
                        let (length, repeat) = {
                            let marker: String = inner.iter().collect();
                            let (length, repeat) = marker.split_once('x').unwrap();
                            let length: usize = length.parse().unwrap();
                            let repeat: usize = repeat.parse().unwrap();
                            (length, repeat)
                        };

                        if end + length >= chars.len() {
                            panic!("{} + {} >= {} (chars.len())", end, length, chars.len());
                        }

                        let compressed = &chars[end + 1..=end + length];
                        buf.append(&mut compressed.repeat(repeat));

                        //Manually increment `i` to continue from where we left off.
                        //The marker has been completely used by this point, so we can
                        //continue like normal after this.
                        i += x.len() + length;
                        continue;
                    }
                    //The marker is not valid. The input *shouldn't* trigger this if my code
                    //is correct. *fingers crossed*
                    _ => {
                        panic!("Malformed marker in input.");
                    }
                }
            }
            //This is not a marker, so just add it to the final output buffer.
            c => buf.push(c),
        }
        i += 1;
    }

    let len_no_blanks = buf.iter().filter(|c| !c.is_whitespace()).count();

    //Unfortunately, p2 can't be done in the same manner as above.
    //'"it's the bomb!", the documentation claims.' - p2 description
    //Fortunately, finding the length is pretty simple :)
    let p2 = calc_markers(&chars);
    (len_no_blanks, p2)
}

fn calc_markers(chars: &[char]) -> usize {
    //Literals encountered that are not part of decompression
    let mut lits = 0i32;

    // let mut markers = Vec::new();

    let mut i = 0;
    while i < chars.len() {
        println!("{lits}");
        match chars[i] {
            //This is the start of a marker
            '(' => {
                //We need to find the end of the marker because markers
                //can have variable widths based on the digits used,
                //i.e "(10x50)" vs "(1x2)". Just mark the current position
                //and increase it until we find the first ')'. This will
                //give us the range in the string from which to extract
                //the complete marker.
                let mut end = i;
                while end < chars.len() && chars[end] != ')' {
                    end += 1;
                }

                //Ensure we found a marker. This will panic if the marker does not
                //finish. Needs testing.
                match &chars[i..=end] {
                    //x: The whole marker with '(' and ')'
                    //inner: Everything between the parens
                    x @ &['(', ref inner @ .., ')'] => {
                        //Build the string for parsing into ints
                        let (length, repeat) = {
                            let marker: String = inner.iter().collect();
                            let (length, repeat) = marker.split_once('x').unwrap();
                            let length: i32 = length.parse().unwrap();
                            let repeat: i32 = repeat.parse().unwrap();
                            (length, repeat)
                        };

                        if end + length as usize >= chars.len() {
                            panic!("{} + {} >= {} (chars.len())", end, length, chars.len());
                        }

                        lits += length * repeat - x.len() as i32;
                        // markers.push((length, repeat));
                        //Manually increment `i` to continue from where we left off.
                        //The marker has been completely used by this point, so we can
                        //continue like normal after this.
                        i += x.len();
                        continue;
                    }
                    //The marker is not valid. The input *shouldn't* trigger this if my code
                    //is correct. *fingers crossed*
                    _ => {
                        panic!("Malformed marker in input.");
                    }
                }
            }
            //This is not a marker, so just add it to the final output buffer.
            c => {
                lits += 1;
                println!("{c} - {lits}");
            }
        }
        i += 1;
    }

    // let mut product = 1;
    // let mut iter = markers.into_iter().rev().peekable();
    // while let Some(marker) = iter.next() {
    //     if iter.peek().is_none() {
    //         product *= marker.1;
    //     } else {
    //         product *= marker.0 * marker.1;
    //     }
    // }
    // println!("{lits} + {product}");
    // (lits + product) as usize
    lits as usize
}

#[cfg(test)]
mod test {

    use super::*;

    // const PROVIDED: &str = "X(8x2)(3x3)ABCY";
    const PROVIDED: &str = "(27x12)(20x12)(13x14)(7x10)(1x12)A";

    #[test]
    fn provided_p1() {
        assert_eq!(241920, solve(PROVIDED).1);
    }
}

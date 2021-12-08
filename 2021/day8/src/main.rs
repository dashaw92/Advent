use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> usize {
    let outputs: Vec<&str> = input.as_ref().lines()
        .inspect(|s| println!("{}", s))
        .filter_map(|s| s.split("|").nth(1))
        .map(str::trim)
        .flat_map(|s| s.split_ascii_whitespace())
        .filter(|&s| {
            [2, 3, 4, 7].contains(&s.len())
        })
        .collect();

    outputs.len()
}

fn solve_p2(input: impl  AsRef<str>) -> usize {
    let input: Vec<&str> = input.as_ref().lines().collect();
    let mut sum = 0;

    for entry in input {
        /*
            0
           1 2
            3
           4 5
            6
         */
        // /shrug
        let mut map: [char; 7] = [' '; 7];
        let mut signals: HashMap<u8, Vec<char>> = HashMap::new();
        while map.iter().any(|c| c == ' ') {

        }
    }

    sum
}

#[cfg(test)]
mod test {

    use super::*;
    /*
    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe

          ddddddd
         c/g     b/e
         c/g     b/e
         c/g     b/e
         c/g     b/e
          c/g
         ?     b/e
         ?     b/e
         ?     b/e
         ?     b/e
          ?????

          fecdb

          dddddddd
         c?      e?
         c?      e?
         c?      e?
         c?      e?
          c?c?cc?
                 e?
                 e?
                 e?
                 e?
          ok I understand this has to do with permutations
          ... which is easily one of my least favorite things
          I'm gonna pass on part 2 lol.
     */

    const PROVIDED: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn provided_p1() {
        assert_eq!(26, solve_p1(PROVIDED));
    }

    // #[test]
    // fn provided_p2() {
    //     assert_eq!(0, solve_p2(PROVIDED));
    // }
}

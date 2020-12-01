use aoc::*;

fn main() {
    let file = read_input("input.txt");
    let input: Vec<i32> = file.lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    println!("Part 1: {}", Part1::solve(&input));
    println!("Part 2: {}", Part2::solve(&input));
}

struct Part1;
struct Part2;

impl Solution<&[i32], i32> for Part1 {

    fn solve(input: &[i32]) -> i32 {
        input.into_iter()
            .filter_map(|x| {
                for y in input {
                    if x + y == 2020 {
                        return Some((x, y))
                    }
                }

                None
            })
            .map(|(x, y)| x * y)
            .nth(0)
            .unwrap()
    }

}

impl Solution<&[i32], i32> for Part2 {

    fn solve(input: &[i32]) -> i32 {
        input.into_iter()
            .filter_map(|x| {
                for y in input {
                    for z in input {
                        if x + y + z == 2020 {
                            return Some((x, y, z))
                        }
                    }
                }

                None
            })
            .map(|(x, y, z)| x * y * z)
            .nth(0)
            .unwrap()
    }

}
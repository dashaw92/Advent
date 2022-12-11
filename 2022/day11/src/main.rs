use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

mod ooh_ooh_ah_ah;
use ooh_ooh_ah_ah::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let (p1, p2) = solve(&input, "\n\n");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(input: impl AsRef<str>, split: &str) -> (usize, usize) {
    let mut monkeys = parse_monkeys(&input, split);
    let mut monkeys_p2 = parse_monkeys(&input, split);

    let lcm: Int = monkeys.iter().map(|monkey| monkey.pred.modulo).product();
    let p1 = run(&mut monkeys, 20, |x| x / 3);
    let p2 = run(&mut monkeys_p2, 10_000, |x| x % lcm);
    (p1, p2)
}

fn run(monkeys: &mut [Monkey], rounds: usize, management: impl Fn(Int) -> Int) -> usize {
    for _ in 0..rounds {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];

            //unfortunately needed because I can't borrow both the current monkey
            //and the "thrown to" monkey at the same time.
            let mut postbag: HashMap<usize, Vec<Int>> = HashMap::new();

            monkey.inspected += monkey.items.len();
            monkey
                .items
                .iter()
                .map(|&it| monkey.op.apply(it))
                .map(&management)
                .for_each(|it| {
                    let id = monkey.pred.test(it);
                    postbag.entry(id).or_default().push(it);
                });

            monkey.items.clear();
            postbag
                .into_iter()
                .for_each(|(id, items)| monkeys[id].items.extend(items));
        }
    }

    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = include_str!("../provided.txt");

    #[test]
    fn provided_p1() {
        assert_eq!((10605, 2713310158), solve(PROVIDED, "\r\n\r\n"));
    }
}

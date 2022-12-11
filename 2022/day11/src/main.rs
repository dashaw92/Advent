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
    let mut monkeys = parse_monkeys(input, split);

    let p1 = run(&mut monkeys, 20);
    (p1, 0)
}

fn run(monkeys: &mut [Monkey], rounds: usize) -> usize {
    print_items(0, monkeys);
    for round in 0..rounds {
        for id in 0..monkeys.len() {
            let monkey = &mut monkeys[id];

            //unfortunately needed because I can't borrow both the current monkey
            //and the "thrown to" monkey at the same time.
            let mut postbag: HashMap<usize, Vec<i32>> = HashMap::new();

            for &item in &monkey.items {
                monkey.inspected += 1;
                let new = monkey.op.apply(item) / 3;
                if new % monkey.test.modulo == 0 {
                    postbag.entry(monkey.test.pass).or_default().push(new);
                } else {
                    postbag.entry(monkey.test.fail).or_default().push(new);
                }
            }

            monkey.items.clear();
            postbag
                .into_iter()
                .for_each(|(id, items)| monkeys[id].items.extend(items));
        }
        print_items(round + 1, monkeys);
    }

    monkey_business(monkeys)
}

fn monkey_business(monkeys: &mut [Monkey]) -> usize {
    monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));
    let bananas = monkeys[0].inspected * monkeys[1].inspected;
    monkeys.sort_by(|a, b| a.id.cmp(&b.id));

    bananas
}

#[cfg(test)]
fn print_items(round: usize, monkeys: &[Monkey]) {
    println!("Round {round}");
    for monkey in monkeys {
        println!(
            "Monkey {} [{}]: {:?}",
            monkey.id, monkey.inspected, monkey.items
        );
    }
    println!();
}

#[cfg(not(test))]
fn print_items(_: usize, _: &[Monkey]) {}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = include_str!("../provided.txt");

    #[test]
    fn provided_p1() {
        assert_eq!((10605, 0), solve(PROVIDED, "\r\n\r\n"));
    }
}

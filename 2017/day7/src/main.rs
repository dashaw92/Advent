use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {:?}", solve(&input));
    Ok(())
}

fn parse_name_weight(s: &str) -> (String, u32) {
    let (name, weight) = s.split_once(' ').unwrap();
    let weight = weight
        .trim()
        .strip_prefix('(')
        .map(|s| {
            if s.contains("->") {
                s.split_once("->").unwrap().0.trim()
            } else {
                s
            }
        })
        .and_then(|s| s.strip_suffix(')'))
        .unwrap()
        .parse()
        .unwrap();
    (name.into(), weight)
}

fn parse_progs(input: &str) -> Vec<Prog> {
    let mut progs = Vec::new();
    for line in input.lines() {
        let (name, weight) = parse_name_weight(line);
        let children = if line.contains("->") {
            let (_, children) = line.split_once("->").unwrap();
            children
                .split(", ")
                .map(str::trim)
                .map(ToOwned::to_owned)
                .collect()
        } else {
            Vec::new()
        };

        progs.push(Prog {
            name,
            weight,
            children,
        });
    }

    progs
}

fn find_parent(progs: &[Prog]) -> String {
    //the root program will have children, but will not appear in the is_child list
    let mut with_children = vec![];
    let mut is_child = vec![];

    for prog in progs {
        if !prog.children.is_empty() {
            with_children.push(prog.name.clone());
        }

        for child in &prog.children {
            is_child.push(child.clone());
        }
    }

    for parent in &with_children {
        if !is_child.contains(parent) {
            return parent.clone();
        }
    }

    unreachable!("The tree must have a parent...")
}

fn find_prog(name: String, progs: &[Prog]) -> &Prog {
    progs.iter().find(|prog| prog.name == name).unwrap()
}

fn calc(prog: &Prog, progs: &[Prog]) -> u32 {
    if prog.children.is_empty() {
        return prog.weight;
    }

    let weights: Vec<u32> = prog
        .children
        .iter()
        .map(|name| find_prog(name.clone(), progs))
        .map(|prog| calc(prog, progs))
        .collect();
    let mut exp = 0;
    for &weight in &weights {
        if exp == 0 {
            exp = weight;
        }

        if exp != weight {
            println!(
                "Found unbalanced weight (base: {} -> {:?}) {exp}, {weight}",
                &prog.name, &weights,
            );
        }
    }

    prog.weight + weights.iter().sum::<u32>()
}

fn solve(input: impl AsRef<str>) -> (String, String) {
    let progs = parse_progs(input.as_ref());
    let p1 = find_parent(&progs);

    let base = find_prog(p1.clone(), &progs);

    dbg!(calc(base, &progs));

    (p1, "".into())
}

#[derive(Debug)]
struct Prog {
    name: String,
    weight: u32,
    children: Vec<String>,
}

// fn solve_p2(input: impl AsRef<str>) -> usize {
//     0
// }

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn provided_p1() {
        assert_eq!(("tknk".to_owned(), "".to_owned()), solve(PROVIDED));
    }

    // #[test]
    // fn provided_p2() {
    //     assert_eq!(0, solve_p2(PROVIDED));
    // }
}

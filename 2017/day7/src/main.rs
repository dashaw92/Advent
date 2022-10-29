use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {:?}", solve(&input));
    Ok(())
}

fn parse_name_weight(s: &str) -> (String, i32) {
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

static mut BAD_WEIGHT_FOUND: bool = false;

fn calc(prog: &Prog, progs: &[Prog]) -> i32 {
    if prog.children.is_empty() {
        return prog.weight;
    }

    let weights: Vec<i32> = prog
        .children
        .iter()
        .map(|name| find_prog(name.clone(), progs))
        .map(|prog| calc(prog, progs))
        .collect();

    //The input has at most one unbalanced program, meaning if we take the first n weights,
    //one of these patterns MUST be true, in which case we know the correct weight, and can
    //then determine the incorrect weight by iterating over the entire weights array and finding
    //the one that is not the same as `exp`.
    let exp = match *weights.as_slice() {
        [x, y, _, ..] if x == y => x,
        [x, _, z, ..] if x == z => x,
        [_, y, z, ..] if y == z => y,
        [x, y] if x == y => x,
        _ => unreachable!("In the first 3 weights, one of the above patterns must match!"),
    };

    for (idx, &weight) in weights.iter().enumerate() {
        if weight != exp && unsafe { !BAD_WEIGHT_FOUND } {
            unsafe {
                BAD_WEIGHT_FOUND = true;
            }

            let prog = find_prog(prog.children[idx].clone(), progs);

            println!(
                "Found unbalanced weight ({} -> {}) {exp}, {weight}.",
                prog.name, prog.weight
            );

            let diff = exp - weight;

            println!(
                "In order for {} to be balanced, its weight must be {}.",
                prog.name,
                prog.weight + diff
            );
        }
    }

    prog.weight + weights.iter().sum::<i32>()
}

fn solve(input: impl AsRef<str>) -> String {
    let progs = parse_progs(input.as_ref());
    let p1 = find_parent(&progs);

    let base = find_prog(p1.clone(), &progs);

    calc(base, &progs);

    p1
}

#[derive(Debug)]
struct Prog {
    name: String,
    weight: i32,
    children: Vec<String>,
}

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
        assert_eq!("tknk".to_owned(), solve(PROVIDED));
    }

    // #[test]
    // fn provided_p2() {
    //     assert_eq!(0, solve_p2(PROVIDED));
    // }
}

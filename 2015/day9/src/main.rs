use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

type TownGraph = HashMap<String, Vec<(String, i32)>>;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    println!("Part 1: {}", solve_p1(&input));
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> i32 {
    let graph = parse_graph(input);

    let mut dists = Vec::new();
    for town in graph.keys().cloned() {
        dists.push(travel(&graph, town));
    }

    dists.into_iter().min().unwrap()
}

fn travel(graph: &TownGraph, start: String) -> i32 {
    println!("starting travel from {start}");
    let mut visited: HashSet<String> = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(start.clone());

    let mut dist = 0;
    while !q.is_empty() {
        let town = q.pop_front().unwrap();
        visited.insert(town.clone());

        for (neighbor, weight) in graph.get(&town).unwrap() {
            if visited.contains(neighbor) {
                continue;
            }
            println!("{town} -> {neighbor} = {weight}");

            q.push_back(neighbor.clone());
            visited.insert(neighbor.clone());
            dist += weight
        }
    }

    println!("ending travel for {start} = {dist}\n{}", "-".repeat(200));
    dist
}

fn parse_graph(input: impl AsRef<str>) -> TownGraph {
    let mut map = HashMap::new();

    for line in input.as_ref().lines() {
        let ((start, end), dist) = line
            .split_once(" = ")
            .map(|(names, distance)| (names.split_once(" to ").unwrap(), distance))
            .unwrap();
        let dist = dist.parse().unwrap();
        let start = start.to_lowercase();
        let end = end.to_lowercase();

        (*map.entry(start.clone()).or_insert(vec![])).push((end.clone(), dist));
        (*map.entry(end.clone()).or_insert(vec![])).push((start.clone(), dist));
    }

    map
}

#[cfg(test)]
mod test {

    use super::*;

    const PROVIDED: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn provided_p1() {
        assert_eq!(605, solve_p1(PROVIDED));
    }
}

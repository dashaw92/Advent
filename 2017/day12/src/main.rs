use std::collections::{HashMap, HashSet};
use std::io::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let programs = read_to_string("input.txt")?
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split = line.split("<->");

            let prog = split.next().unwrap().trim().to_owned();
            let connections: HashSet<_> = split.next()
                .map(|list| list.split(", "))
                .unwrap()
                .map(|connect| connect.trim().to_owned())
                .collect();
            
            (prog, connections)
        }).collect::<HashMap<_, _>>();

    println!("{:#?}", programs);
    Ok(())
}

mod wire;

use aoc::*;
use wire::*;

fn main() {
    let input = read_input("input.txt");
    let lines: Vec<_> = input.lines().collect();

    let mut wires = Vec::new();

    for line in lines {
        let mut wire = Wire::new();
        line.split_terminator(",")
            .flat_map(|x| x.parse().ok())
            .for_each(|mov| wire.take_move(mov));

        wires.push(wire);
    }

    println!("Matches: {:?}", Wire::find_matches(wires[0].clone(), wires[1].clone()));
}

use std::{collections::HashSet, str::FromStr};

use aoc::*;

fn main() {
    let input = read_input("input.txt");
    let mut cpu: Cpu = input.into();

    loop {
        match cpu.run_part_1() {
            Some(acc) => {
                println!("Part 1: Found the infinite loop. Acc is {}", acc);
                break;
            }
            _ => continue,
        }
    }
}

enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut split = input.split(' ');
        match (split.next(), split.next().and_then(|val| val.parse().ok())) {
            (Some(opcode), Some(value)) => {
                Ok(match opcode {
                    "nop" => Instruction::Nop(value),
                    "jmp" => Instruction::Jmp(value),
                    "acc" => Instruction::Acc(value),
                    _ => return Err(()),
                })
            },
            _ => Err(())
        }
    }

}

struct Cpu {
    tape: Vec<Instruction>,
    acc: i32,
    history: HashSet<usize>,
    pc: usize,
}

impl From<String> for Cpu {

    fn from(tape: String) -> Self {
        let tape: Vec<Instruction> = tape.lines()
            .filter_map(|line| line.parse().ok())
            .collect();

        Self {
            tape,
            acc: 0,
            history: HashSet::new(),
            pc: 0,
        }
    }

}

impl Cpu {

    fn run_part_1(&mut self) -> Option<i32> {
        if !self.history.insert(self.pc) {
            return Some(self.acc);
        }

        let instruction = &self.tape[self.pc];
        match *instruction {
            Instruction::Nop(_) => {},
            Instruction::Jmp(distance) => {
                self.pc = (self.pc as i32 + distance) as usize;
                return None;
            }
            Instruction::Acc(amount) => self.acc += amount,
        }

        self.pc += 1;
        None
    }

}
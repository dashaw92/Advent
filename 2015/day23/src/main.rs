use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let mem = solve_p1(&input);
    println!("Part 1: {}", mem.reg[1]);

    let p2 = format!("inc a\n{}", input);
    let mem = solve_p1(&p2);
    println!("Part 2: {}", mem.reg[1]);
    Ok(())
}

fn solve_p1(input: impl AsRef<str>) -> Mem {
    let tape = convert(input);

    let mut mem = Mem { reg: [0; 2], ip: 0 };

    while mem.ip >= 0 && mem.ip < tape.len() as i32 {
        let op = &tape[mem.ip as usize];

        let mut jmp = 1;
        match op {
            Op::Hlf(r) => mem.reg[*r as usize] /= 2,
            Op::Tpl(r) => mem.reg[*r as usize] *= 3,
            Op::Inc(r) => mem.reg[*r as usize] += 1,
            Op::Jmp(off) => jmp = *off,
            Op::Jie(r, off) => {
                if mem.reg[*r as usize] & 1 == 0 {
                    jmp = *off;
                }
            }
            Op::Jio(r, off) => {
                if mem.reg[*r as usize] == 1 {
                    jmp = *off;
                }
            }
        }

        mem.ip += jmp;
    }

    mem
}

enum Op {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}

type Reg = usize;

struct Mem {
    reg: [i64; 2],
    ip: i32,
}

fn convert(input: impl AsRef<str>) -> Vec<Op> {
    let mut ops = Vec::new();
    for instr in input.as_ref().lines() {
        let (op, rest) = instr.split_once(' ').unwrap();
        let ch = rest.chars().next().unwrap();
        match op {
            "hlf" => ops.push(Op::Hlf(to_reg(ch))),
            "tpl" => ops.push(Op::Tpl(to_reg(ch))),
            "inc" => ops.push(Op::Inc(to_reg(ch))),
            "jmp" => ops.push(Op::Jmp(rest.parse().unwrap())),
            "jie" => {
                let (reg, offset) = rest.split_once(", ").unwrap();
                let reg = to_reg(reg.chars().next().unwrap());
                let offset = offset.parse().unwrap();
                ops.push(Op::Jie(reg, offset));
            }
            "jio" => {
                let (reg, offset) = rest.split_once(", ").unwrap();
                let reg = to_reg(reg.chars().next().unwrap());
                let offset = offset.parse().unwrap();
                ops.push(Op::Jio(reg, offset));
            }
            _ => {}
        }
    }
    ops
}

fn to_reg(id: char) -> usize {
    match id {
        'a' => 0,
        'b' => 1,
        _ => panic!("invalid reg"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn test() {
        let mem = solve_p1(EX);
        assert_eq!(2, mem.reg[0]);
    }
}

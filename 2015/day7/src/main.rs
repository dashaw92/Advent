#![allow(dead_code)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input.txt")?;

    let mut circuit = Circuit::from_input(input);
    let out_a = circuit.eval("a", &[]);
    println!("Wire 'a' has the value {out_a}");
    let overrides = vec![("b", out_a)];
    circuit.reset();
    let out_a = circuit.eval("a", &overrides);
    println!("Wire 'a' has the value {out_a} after overriding 'b'!");
    Ok(())
}

#[derive(Debug)]
struct Circuit {
    wires: HashMap<String, Wire>,
    original: HashMap<String, Wire>,
}

#[derive(Debug, Clone)]
struct Wire {
    state: State,
}

#[derive(Debug, Clone)]
enum State {
    Value(u16),
    Instr(Box<Instruction>),
}

#[derive(Debug, Clone)]
enum Instruction {
    Value(u16),
    Assign(Target),
    And(Target, Target),
    Or(Target, Target),
    Lshift(Target, u16),
    Rshift(Target, u16),
    Not(Target),
}

#[derive(Debug, Clone)]
enum Target {
    Val(u16),
    Var(String),
}

impl<S: AsRef<str>> From<S> for Target {
    fn from(s: S) -> Self {
        if let Ok(int) = s.as_ref().parse() {
            return Target::Val(int);
        }

        Target::Var(s.as_ref().into())
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn binary_op_parse<'a, 'b>(input: &'a str, op: &'b str) -> (&'a str, &'a str) {
            input.split_once(op).unwrap()
        }

        if s.contains("AND") {
            let (w_in, w_out) = binary_op_parse(s, " AND ");
            return Ok(Instruction::And(w_in.into(), w_out.into()));
        }

        if s.contains("OR") {
            let (w_in, w_out) = binary_op_parse(s, " OR ");
            return Ok(Instruction::Or(w_in.into(), w_out.into()));
        }

        if s.contains("LSHIFT") {
            let (w_in, val) = binary_op_parse(s, " LSHIFT ");
            return Ok(Instruction::Lshift(w_in.into(), val.parse().unwrap()));
        }

        if s.contains("RSHIFT") {
            let (w_in, val) = binary_op_parse(s, " RSHIFT ");
            return Ok(Instruction::Rshift(w_in.into(), val.parse().unwrap()));
        }

        if s.contains("NOT") {
            let (_, wire) = s.split_once(' ').unwrap();
            return Ok(Instruction::Not(wire.into()));
        }

        if s.chars().next().unwrap().is_ascii_digit() {
            let val = s.parse().unwrap();
            return Ok(Instruction::Value(val));
        }

        Ok(Instruction::Assign(s.into()))
    }
}

impl Circuit {
    fn from_input(input: impl AsRef<str>) -> Self {
        let mut wires = HashMap::new();
        for instr in input.as_ref().lines() {
            let (lhs, out) = instr.split_once(" -> ").unwrap();
            let instr: Instruction = lhs.parse().unwrap();
            let wire = Wire {
                state: State::Instr(Box::new(instr)),
            };

            wires.insert(out.into(), wire);
        }

        let original = wires.clone();

        Self { wires, original }
    }

    fn reset(&mut self) {
        self.wires = self.original.clone();
    }

    fn update_wire(&mut self, name: impl AsRef<str>, value: u16) -> u16 {
        self.wires.insert(
            name.as_ref().into(),
            Wire {
                state: State::Value(value),
            },
        );

        value
    }

    fn eval(&mut self, wire_name: impl AsRef<str>, overrides: &[(&str, u16)]) -> u16 {
        for (w_override, val) in overrides {
            if w_override == &wire_name.as_ref() {
                return *val;
            }
        }

        let wire = self.wires.get_mut(wire_name.as_ref()).unwrap().clone();
        match &wire.state {
            State::Value(val) => *val,
            State::Instr(instr) => match instr.borrow() {
                Instruction::Value(val) => {
                    let v = *val;
                    self.update_wire(wire_name, v);
                    v
                }
                Instruction::Assign(target) => match target {
                    Target::Val(_) => unreachable!("This should never happen!"),
                    Target::Var(var) => {
                        let val = self.eval(var, overrides);
                        self.update_wire(wire_name, val);
                        val
                    }
                },
                Instruction::And(w_lhs, w_rhs) => match (w_lhs, w_rhs) {
                    (Target::Val(i), Target::Var(wire)) => {
                        let val = self.eval(wire, overrides) & i;
                        self.update_wire(wire_name, val);
                        val
                    }
                    (Target::Var(lhs), Target::Var(rhs)) => {
                        let lhs = self.eval(lhs, overrides);
                        let rhs = self.eval(rhs, overrides);
                        let val = lhs & rhs;
                        self.update_wire(wire_name, val);
                        val
                    }
                    _ => unreachable!("AND never has a numeric RHS."),
                },
                Instruction::Or(w_lhs, w_rhs) => match (w_lhs, w_rhs) {
                    (Target::Val(i), Target::Var(wire)) => {
                        let val = self.eval(wire, overrides) | i;
                        self.update_wire(wire_name, val);
                        val
                    }
                    (Target::Var(lhs), Target::Var(rhs)) => {
                        let lhs = self.eval(lhs, overrides);
                        let rhs = self.eval(rhs, overrides);
                        let val = lhs | rhs;
                        self.update_wire(wire_name, val);
                        val
                    }
                    _ => unreachable!("OR never has a numeric RHS."),
                },
                Instruction::Lshift(w_lhs, rhs) => match w_lhs {
                    Target::Var(lhs) => {
                        let lhs = self.eval(lhs, overrides);
                        let val = lhs << rhs;
                        self.update_wire(wire_name, val);
                        val
                    }
                    _ => unreachable!("LSHIFT never has a numeric LHS."),
                },
                Instruction::Rshift(w_lhs, rhs) => match w_lhs {
                    Target::Var(lhs) => {
                        let lhs = self.eval(lhs, overrides);
                        let val = lhs >> rhs;
                        self.update_wire(wire_name, val);
                        val
                    }
                    _ => unreachable!("RSHIFT never has a numeric LHS."),
                },
                Instruction::Not(wire) => match wire {
                    Target::Var(wire) => {
                        let wire = self.eval(wire, overrides);
                        let val = !wire;
                        self.update_wire(wire_name, val);
                        val
                    }
                    _ => unreachable!("NOT never has a numeric value."),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDED: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_provided() {
        let expected: Vec<(&str, u16)> = vec![
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        let mut circuit = Circuit::from_input(PROVIDED);

        for (name, value) in &expected {
            assert_eq!(*value, circuit.eval(name, &[]));
        }
    }
}

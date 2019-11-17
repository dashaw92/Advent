use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

//Represents an operation supported by this virtual machine
enum Op {
    //cpy <value> <register>, value = register or integer
    Cpy(String, char),
    //inc <register>
    Inc(char),
    //dec <register>
    Dec(char),
    //jnz <value> <jump>, value = register or integer, jump = relative jump to
    Jnz(String, i32),
    //invalid line encountered while parsing, ignored.
    Error,
}

impl Op {
    //Parse a str into an Op. "cpy 2 a" => Op::Cpy("2", 'a')
    fn from(line: &str) -> Self {
        //This is intentionally left unsafe.

        let parts: Vec<_> = line.split_whitespace().collect();

        let op = parts[0];
        match op {
            "cpy" => {
                let val = parts[1].to_string();
                let reg = parts[2].chars().nth(0).unwrap();
                Op::Cpy(val, reg)
            },
            "inc" => {
                let reg = parts[1].chars().nth(0).unwrap();
                Op::Inc(reg)
            },
            "dec" => {
                let reg = parts[1].chars().nth(0).unwrap();
                Op::Dec(reg)
            },
            "jnz" => {
                let reg = parts[1].to_string();
                let val = parts[2].parse().unwrap();
                Op::Jnz(reg, val)
            },
            //Default case. Invalid opcode.
            _ => Op::Error,
        }
    }
}

struct Program {
    program: Vec<Op>,
    pointer: i32,
    registers: HashMap<char, i32>,
}

impl Program {
    //Parses the input and creates the Program
    fn from_input(input: &str) -> Self {
        let program = input.lines().map(Op::from).collect();

        Program {
            program,
            pointer: 0,
            registers: HashMap::new(),
        }
    }

    //Solves instead for part 2.
    #[allow(dead_code)]
    fn part2(&mut self) {
        self.registers.insert('c', 1);
    }

    //Execute the program completely.
    fn execute(&mut self) {
        while self.pointer < self.program.len() as i32 {

            //The current operation at the pointer
            let code = &self.program[self.pointer as usize];
            match code {
                //Increment the register by one
                Op::Inc(reg) => *self.registers.entry(*reg).or_insert(0) += 1,
                //Decrement the register by one
                Op::Dec(reg) => *self.registers.entry(*reg).or_insert(0) -= 1,
                //Set the register to be the value
                Op::Cpy(value, reg) => {
                    //Is value a raw integer?
                    if value.chars().nth(0).unwrap().is_digit(10) {
                        //Overwrite the register's old value with the new one.
                        self.registers.insert(*reg, value.parse().unwrap());
                    }
                    //value is another register.
                    else {
                        //Copy the value of the other register into `reg`
                        let other = value.chars().nth(0).unwrap();
                        let other_val = *self.registers.entry(other).or_insert(0);
                        self.registers.insert(*reg, other_val);
                    }
                    
                },
                //Move the pointer by `value` relative to the pointer,
                //if `reg` is not zero. (jump not zero)
                Op::Jnz(reg, value) => {
                    let first = reg.chars().nth(0).unwrap();

                    //Is `reg` a raw integer?
                    let val = if first.is_digit(10) {
                        reg.parse().unwrap()
                    } 
                    else {
                        //No, `reg` is a register in the program.
                        *self.registers.entry(first).or_insert(0)
                    };

                    if val != 0 {
                        //Relative jump
                        self.pointer += value;
                        //Avoid adding 1 to the pointer after this iteration
                        continue;
                    }
                },
                //Op::Err
                _ => (),
            }

            //On to the next iteration.
            self.pointer += 1;
        }
    }

    fn get_register(&self, reg: char) -> Option<&i32> {
        self.registers.get(&reg)
    }
}

fn main() {
    let mut program = Program::from_input(INPUT);
    //program.part2();
    program.execute();
    println!("The value of the register {} is {}", 'a', program.get_register('a').unwrap());
}
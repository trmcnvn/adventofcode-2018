use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

#[derive(Debug, Clone)]
struct Program {
    registers: [usize; 6],
    instruction_pointer: usize,
    instructions: Vec<(Opcode, [usize; 3])>,
}

impl Program {
    fn handle_opcode(&self, opcode: &Opcode, a: usize, b: usize) -> usize {
        match opcode {
            Opcode::addr => self.registers[a] + self.registers[b],
            Opcode::addi => self.registers[a] + b,
            Opcode::mulr => self.registers[a] * self.registers[b],
            Opcode::muli => self.registers[a] * b,
            Opcode::banr => self.registers[a] & self.registers[b],
            Opcode::bani => self.registers[a] & b,
            Opcode::borr => self.registers[a] | self.registers[b],
            Opcode::bori => self.registers[a] | b,
            Opcode::setr => self.registers[a],
            Opcode::seti => a,
            Opcode::gtir => {
                if a > self.registers[b] {
                    1
                } else {
                    0
                }
            }
            Opcode::gtri => {
                if self.registers[a] > b {
                    1
                } else {
                    0
                }
            }
            Opcode::gtrr => {
                if self.registers[a] > self.registers[b] {
                    1
                } else {
                    0
                }
            }
            Opcode::eqir => {
                if a == self.registers[b] {
                    1
                } else {
                    0
                }
            }
            Opcode::eqri => {
                if self.registers[a] == b {
                    1
                } else {
                    0
                }
            }
            Opcode::eqrr => {
                if self.registers[a] == self.registers[b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Program {
    let mut program = Program {
        registers: [0, 0, 0, 0, 0, 0],
        instruction_pointer: 0,
        instructions: Vec::new(),
    };
    let re = Regex::new(r"(\w{4})\s(\d+)\s(\d+)\s(\d+)").unwrap();
    for line in input.lines() {
        let bytes = line.as_bytes();
        if bytes[0] == b'#' {
            program.instruction_pointer = (bytes[4] as char).to_digit(10).unwrap() as usize;
            continue;
        }
        let caps = re.captures(line).unwrap();
        let instructions: [usize; 3] = [
            caps[2].parse().unwrap(),
            caps[3].parse().unwrap(),
            caps[4].parse().unwrap(),
        ];
        let instruction = match &caps[1] {
            "addr" => (Opcode::addr, instructions),
            "addi" => (Opcode::addi, instructions),
            "mulr" => (Opcode::mulr, instructions),
            "muli" => (Opcode::muli, instructions),
            "banr" => (Opcode::banr, instructions),
            "bani" => (Opcode::bani, instructions),
            "borr" => (Opcode::borr, instructions),
            "bori" => (Opcode::bori, instructions),
            "setr" => (Opcode::setr, instructions),
            "seti" => (Opcode::seti, instructions),
            "gtir" => (Opcode::gtir, instructions),
            "gtri" => (Opcode::gtri, instructions),
            "gtrr" => (Opcode::gtrr, instructions),
            "eqir" => (Opcode::eqir, instructions),
            "eqri" => (Opcode::eqri, instructions),
            "eqrr" => (Opcode::eqrr, instructions),
            _ => unreachable!(),
        };
        program.instructions.push(instruction);
    }
    program
}

/// What value is left in register 0 when the background process halts?
#[aoc(day19, part1)]
fn solve_part1(program: &Program) -> usize {
    let mut our_program = program.to_owned();
    let register_bound = our_program.instruction_pointer;
    our_program.instruction_pointer = our_program.registers[register_bound];
    while our_program.instruction_pointer < our_program.instructions.len() {
        our_program.registers[register_bound] = our_program.instruction_pointer;
        let instruction = &our_program.instructions[our_program.instruction_pointer];
        let [a, b, c] = instruction.1;
        let result = our_program.handle_opcode(&instruction.0, a, b);
        our_program.registers[c] = result;
        our_program.instruction_pointer = our_program.registers[register_bound] + 1;
    }
    our_program.registers[0]
}

/// What value is left in register 0 when this new background process halts?
#[aoc(day19, part2)]
fn solve_part2(program: &Program) -> usize {
    let mut our_program = program.to_owned();
    let register_bound = our_program.instruction_pointer;
    our_program.registers[0] = 1;
    our_program.instruction_pointer = our_program.registers[register_bound];
    while our_program.registers[register_bound] != 1 {
        our_program.registers[register_bound] = our_program.instruction_pointer;
        let instruction = &our_program.instructions[our_program.instruction_pointer];
        let [a, b, c] = instruction.1;
        let result = our_program.handle_opcode(&instruction.0, a, b);
        our_program.registers[c] = result;
        our_program.instruction_pointer = our_program.registers[register_bound] + 1;
    }

    let target = *our_program.registers.iter().max().unwrap();
    let mut total = 0;
    for i in 1..=target {
        if target % i == 0 {
            total += i;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::day19::*;

    #[test]
    fn part1() {
        let input = input_generator(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn part2() {
        let input = input_generator(
            "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 1);
    }
}

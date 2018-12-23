use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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

#[aoc_generator(day21)]
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

/// What is the lowest non-negative integer value for register 0 that causes the program
/// to halt after executing the fewest instructions?
#[aoc(day21, part1)]
fn solve_part1(program: &Program) -> usize {
    let mut our_program = program.to_owned();
    let register_bound = our_program.instruction_pointer;
    our_program.instruction_pointer = our_program.registers[register_bound];
    while our_program.instruction_pointer < our_program.instructions.len() {
        our_program.registers[register_bound] = our_program.instruction_pointer;
        let instruction = &our_program.instructions[our_program.instruction_pointer];
        let [a, b, c] = instruction.1;
        if instruction.0 == Opcode::eqrr && b == 0 {
            return our_program.registers[a];
        }
        let result = our_program.handle_opcode(&instruction.0, a, b);
        our_program.registers[c] = result;
        our_program.instruction_pointer = our_program.registers[register_bound] + 1;
    }
    unreachable!();
}

/// What is the lowest non-negative integer value for register 0 that causes the program
/// to halt after executing the most instructions?
#[aoc(day21, part2, slow)]
fn solve_part2(program: &Program) -> usize {
    let mut values_seen = HashSet::new();
    let mut last_value = 0;

    let mut our_program = program.to_owned();
    let register_bound = our_program.instruction_pointer;
    our_program.instruction_pointer = our_program.registers[register_bound];
    while our_program.instruction_pointer < our_program.instructions.len() {
        our_program.registers[register_bound] = our_program.instruction_pointer;
        let instruction = &our_program.instructions[our_program.instruction_pointer];
        let [a, b, c] = instruction.1;
        if instruction.0 == Opcode::eqrr && b == 0 {
            if !values_seen.insert(our_program.registers[a]) {
                return last_value;
            }
            last_value = our_program.registers[a];
        }
        let result = our_program.handle_opcode(&instruction.0, a, b);
        our_program.registers[c] = result;
        our_program.instruction_pointer = our_program.registers[register_bound] + 1;
    }
    unreachable!();
}

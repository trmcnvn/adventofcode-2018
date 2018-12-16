use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
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

impl Opcode {
    fn values() -> Vec<Opcode> {
        vec![
            Opcode::addr,
            Opcode::addi,
            Opcode::mulr,
            Opcode::muli,
            Opcode::banr,
            Opcode::bani,
            Opcode::borr,
            Opcode::bori,
            Opcode::setr,
            Opcode::seti,
            Opcode::gtir,
            Opcode::gtri,
            Opcode::gtrr,
            Opcode::eqir,
            Opcode::eqri,
            Opcode::eqrr,
        ]
    }
}

/// (before Before, instruction, before After)
type Sample = (Vec<usize>, Vec<usize>, Vec<usize>);
type AOCResult = (Vec<Sample>, Vec<Vec<usize>>);

fn handle_opcode(opcode: Opcode, before: &[usize], instruction: &[usize]) -> usize {
    match opcode {
        Opcode::addr => before[instruction[1]] + before[instruction[2]],
        Opcode::addi => before[instruction[1]] + instruction[2],
        Opcode::mulr => before[instruction[1]] * before[instruction[2]],
        Opcode::muli => before[instruction[1]] * instruction[2],
        Opcode::banr => before[instruction[1]] & before[instruction[2]],
        Opcode::bani => before[instruction[1]] & instruction[2],
        Opcode::borr => before[instruction[1]] | before[instruction[2]],
        Opcode::bori => before[instruction[1]] | instruction[2],
        Opcode::setr => before[instruction[1]],
        Opcode::seti => instruction[1],
        Opcode::gtir => {
            if instruction[1] > before[instruction[2]] {
                1
            } else {
                0
            }
        }
        Opcode::gtri => {
            if before[instruction[1]] > instruction[2] {
                1
            } else {
                0
            }
        }
        Opcode::gtrr => {
            if before[instruction[1]] > before[instruction[2]] {
                1
            } else {
                0
            }
        }
        Opcode::eqir => {
            if instruction[1] == before[instruction[2]] {
                1
            } else {
                0
            }
        }
        Opcode::eqri => {
            if before[instruction[1]] == instruction[2] {
                1
            } else {
                0
            }
        }
        Opcode::eqrr => {
            if before[instruction[1]] == before[instruction[2]] {
                1
            } else {
                0
            }
        }
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> AOCResult {
    let re: Regex = Regex::new(r".+:\s{1,2}\[(\d{1}), (\d{1}), (\d{1}), (\d{1})\]").unwrap();
    let mut samples: Vec<Sample> = Vec::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    // Get Samples
    for lines in lines.chunks(4) {
        if !lines[0].contains("Before:") {
            break;
        }

        let before_groups = re.captures(lines[0]).unwrap();
        let before: Vec<usize> = before_groups
            .iter()
            .skip(1)
            .take(4)
            .map(|v| v.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        let instruction: Vec<usize> = lines[1]
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        let after_groups = re.captures(lines[2]).unwrap();
        let after: Vec<usize> = after_groups
            .iter()
            .skip(1)
            .take(4)
            .map(|v| v.unwrap().as_str().parse::<usize>().unwrap())
            .collect();

        samples.push((before, instruction, after));
    }

    // Get Test Code
    for lines in lines.iter().skip(samples.len() * 4) {
        if lines.is_empty() {
            continue;
        }

        let instruction: Vec<usize> = lines
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        instructions.push(instruction);
    }

    (samples, instructions)
}

/// how many samples in your puzzle input behave like three or more opcodes?
#[aoc(day16, part1)]
fn solve_part1(input: &AOCResult) -> usize {
    let mut total_count = 0;
    for sample in &input.0 {
        let mut sample_count = 0;
        let (before, instruction, after) = sample;
        for opcode in 0..16 {
            let result = handle_opcode(Opcode::values()[opcode].clone(), before, instruction);
            let mut new_after = before.to_owned();
            new_after[instruction[3]] = result;
            if new_after == *after {
                sample_count += 1;
            }
        }
        if sample_count >= 3 {
            total_count += 1;
        }
    }
    total_count
}

/// What value is contained in register 0 after executing the test program?
#[aoc(day16, part2)]
fn solve_part2(input: &AOCResult) -> usize {
    let mut candidates = Vec::new();
    for opcode in &Opcode::values() {
        let mut possibilities = Vec::new();
        for n in 0..16 {
            let missed = input.0.iter().filter(|s| s.1[0] == n).any(|s| {
                let result = handle_opcode(opcode.clone(), &s.0, &s.1);
                let mut new_after = s.0.to_owned();
                new_after[s.1[3]] = result;
                new_after != s.2
            });
            if !missed {
                possibilities.push((n, opcode.clone()));
            }
        }
        candidates.push(possibilities);
    }
    let mut opcodes = HashMap::new();
    while opcodes.len() < 16 {
        for possibilities in &candidates {
            let new: Vec<(usize, Opcode)> = possibilities.iter().cloned().filter(|p| !opcodes.contains_key(&p.0)).collect();
            if new.len() == 1 {
                opcodes.insert(new[0].0, new[0].1.clone());
            }
        }
    }

    let mut registers = vec![0, 0, 0, 0];
    for instruction in &input.1 {
        if let Some(opcode) = opcodes.get(&instruction[0]) {
            let result = handle_opcode(opcode.clone(), &registers, &instruction);
            registers[instruction[3]] = result;
        }
    }
    registers[0]
}

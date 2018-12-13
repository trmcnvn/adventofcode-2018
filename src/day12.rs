use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use regex::Regex;
use std::str;

#[derive(Default)]
pub struct Data {
    pub state: HashSet<isize>,
    pub rules: HashSet<Vec<u8>>,
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Box<Data> {
    let mut re = Regex::new("initial state: (.+)+\n").unwrap();
    let caps = re.captures(input).unwrap();

    let mut data = Data::default();
    for (index, pot) in caps[1].chars().enumerate() {
        if pot == '#' {
            data.state.insert(index as isize);
        }
    }

    re = Regex::new("(.+)+ => (#|.)").unwrap();
    for rule in input.lines().skip(1) {
        if rule.is_empty() {
            continue;
        }
        let caps = re.captures(rule).unwrap();
        if &caps[2] == "#" {
            data.rules.insert(caps[1].as_bytes().to_vec());
        }
    }
    Box::new(data)
}

fn next_state(state: &HashSet<isize>, rules: &HashSet<Vec<u8>>) -> HashSet<isize> {
    let min = *state.iter().min().unwrap() as isize - 3;
    let max = *state.iter().max().unwrap() as isize + 3;

    let mut set: HashSet<isize> = HashSet::new();
    for i in min..=max {
        let pattern: Vec<u8> = (0..5)
            .map(|j| {
                let index: isize = i + j - 2;
                if state.contains(&index) {
                    b'#'
                } else {
                    b'.'
                }
            })
            .collect();
        if rules.contains(&pattern) {
            set.insert(i);
        }
    }
    set
}

fn normalize(state: &HashSet<isize>) -> HashSet<isize> {
    let min = state.iter().min().unwrap();
    let mut new_state = HashSet::new();
    for value in state {
        new_state.insert(value - min);
    }
    new_state
}

#[aoc(day12, part1)]
pub fn solve_part1(data: &Data) -> usize {
    let mut state = data.state.clone();
    for _ in 0..20 {
        state = next_state(&state, &data.rules);
    }
    state.iter().sum::<isize>() as usize
}

#[aoc(day12, part2)]
pub fn solve_part2(data: &Data) -> usize {
    let mut state = data.state.clone();
    let mut last_state = HashSet::new();
    for generation in 0_usize..50_000_000_000 {
        state = next_state(&state, &data.rules);
        if !last_state.is_empty() && normalize(&last_state) == normalize(&state) {
            let a: usize = state.iter().map(|&x| x as usize).sum();
            let b: usize = last_state.iter().map(|&x| x as usize).sum();
            let diff = a - b;
            return a + diff * (50_000_000_000 - generation) - diff;
        }
        last_state = state.clone();
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn part_1() {
        let input = input_generator(
            "initial state: #..#.#..##......###...###
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 325);
    }

    #[test]
    fn part_2() {
        let input = input_generator(
            "initial state: #..#.#..##......###...###
...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 999999999374);
    }
}

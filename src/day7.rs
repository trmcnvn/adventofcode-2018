use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use regex::Regex;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();
    let mut pairs: Vec<Vec<_>> = Vec::new();

    for line in input.trim().lines() {
        let caps = re.captures(line).unwrap();
        let (first, second): (char, char) =
            (caps[1].as_bytes()[0] as char, caps[2].as_bytes()[0] as char);
        pairs.push(vec![first, second]);
    }

    pairs
}

#[aoc(day7, part1)]
pub fn solve_1(pairs: &[Vec<char>]) -> String {
    let mut todo: Vec<char> = pairs.iter().cloned().flatten().collect();
    todo.sort();
    todo.dedup();
    let mut done: Vec<char> = Vec::new();

    while !todo.is_empty() {
        let mut valid_tasks: Vec<_> = todo
            .iter()
            .filter(|c| {
                if done.contains(c) {
                    return false;
                }
                for pair in pairs {
                    let slice = pair.as_slice();
                    if c.eq_ignore_ascii_case(&slice[1]) && !done.contains(&slice[0]) {
                        return false;
                    }
                }
                true
            })
            .collect();
        valid_tasks.sort();

        let valid_task = *valid_tasks[0];
        done.push(valid_task);
        todo.remove_item(&valid_task);
    }

    done.iter().collect()
}

#[aoc(day7, part2)]
pub fn solve_2(pairs: &[Vec<char>]) -> usize {
    part2_solver(pairs, 5, 60)
}

fn part2_solver(pairs: &[Vec<char>], worker_count: usize, duration: usize) -> usize {
    let mut total_time: usize = 0;
    let mut workers: HashMap<char, usize> = HashMap::new();

    let mut todo: Vec<char> = pairs.iter().cloned().flatten().collect();
    todo.sort();
    todo.dedup();
    let mut done: Vec<char> = Vec::new();

    while !todo.is_empty() {
        let youre_not_smart = todo.clone();
        let mut valid_tasks: Vec<_> = youre_not_smart
            .iter()
            .filter(|c| {
                if done.contains(c) {
                    return false;
                }
                for pair in pairs {
                    let slice = pair.as_slice();
                    if c.eq_ignore_ascii_case(&slice[1]) && !done.contains(&slice[0]) {
                        return false;
                    }
                }
                true
            })
            .collect();
        valid_tasks.sort();

        for task in valid_tasks {
            if workers.len() < worker_count {
                let work_time = total_time + duration + (*task as u8 - 64) as usize;
                if !workers.contains_key(&task) {
                    workers.insert(*task, work_time);
                }
            }
        }

        for (task, work_time) in workers.clone().iter() {
            if *work_time == (total_time + 1) {
                workers.remove(task);
                done.push(*task);
                todo.remove_item(&task);
            }
        }

        total_time += 1;
    }
    total_time as usize
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    #[test]
    fn part1() {
        let result = input_generator(
            r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
        );
        assert_eq!(solve_1(&result), "CABDFE");
    }

    #[test]
    fn part2() {
        let result = input_generator(
            r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
        );
        assert_eq!(part2_solver(&result, 2, 0), 15);
    }
}

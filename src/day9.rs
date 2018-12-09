use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::VecDeque;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> [usize; 2] {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = re.captures(input).unwrap();
    [caps[1].parse().unwrap(), caps[2].parse().unwrap()]
}

#[aoc(day9, part1)]
pub fn solve_part1(data: &[usize]) -> usize {
    solver(data[0], data[1])
}

#[aoc(day9, part2)]
pub fn solve_part2(data: &[usize]) -> usize {
    solver(data[0], data[1] * 100)
}

fn solver(players: usize, points: usize) -> usize {
    let mut marbles: VecDeque<usize> = VecDeque::new();
    let mut highscores: Vec<usize> = vec![0; players];
    marbles.push_back(0);

    for (marble, player) in (1..=points).zip((0..players).cycle()) {
        if marble % 23 == 0 {
            for _ in 0..7 {
                let value = marbles.pop_front().unwrap();
                marbles.push_back(value);
            }

            let removed_marble = marbles.pop_back().unwrap();
            highscores[player] += marble + removed_marble;
            continue;
        }

        for _ in 0..2 {
            let value = marbles.pop_back().unwrap();
            marbles.push_front(value);
        }
        marbles.push_back(marble);
    }
    *highscores.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day9::*;

    #[test]
    fn part1() {
        let mut input = input_generator("9 players; last marble is worth 25 points");
        assert_eq!(solve_part1(&input), 32);

        input = input_generator("10 players; last marble is worth 1618 points");
        assert_eq!(solve_part1(&input), 8317);

        input = input_generator("13 players; last marble is worth 7999 points");
        assert_eq!(solve_part1(&input), 146_373);

        input = input_generator("17 players; last marble is worth 1104 points");
        assert_eq!(solve_part1(&input), 2764);

        input = input_generator("21 players; last marble is worth 6111 points");
        assert_eq!(solve_part1(&input), 54718);

        input = input_generator("30 players; last marble is worth 5807 points");
        assert_eq!(solve_part1(&input), 37305);
    }

    #[test]
    fn part2() {
        let input = input_generator("9 players; last marble is worth 25 points");
        assert_eq!(solve_part2(&input), 22563);
    }
}

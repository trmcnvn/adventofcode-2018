use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::FromIterator;
use std::collections::VecDeque;
use hashbrown::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(isize, isize, isize, isize);

impl Coord {
    fn distance(&self, other: &Coord) -> usize {
        let a = (self.0 - other.0).abs();
        let b = (self.1 - other.1).abs();
        let c = (self.2 - other.2).abs();
        let d = (self.3 - other.3).abs();
        (a + b + c + d) as usize
    }
}

impl FromIterator<isize> for Coord {
    fn from_iter<I: IntoIterator<Item=isize>>(iter: I) -> Self {
        let mut values = [0_isize; 4];
        for (idx, value) in iter.into_iter().enumerate() {
            values[idx] = value;
        }
        Coord(values[0], values[1], values[2], values[3])
    }
}

#[aoc_generator(day25)]
fn input_generator(input: &str) -> Vec<Coord> {
    let mut coords = Vec::new();
    for line in input.lines() {
        let values = line.trim().split(',').map(|s| s.parse::<isize>().unwrap()).collect::<Coord>();
        coords.push(values);
    }
    coords
}

/// How many constellations are formed by the fixed points in spacetime?
#[aoc(day25, part1)]
fn solve_part1(coords: &[Coord]) -> usize {
    let mut all_points = HashSet::new();
    let mut total_constellations = 0;
    loop {
        let mut tried = HashSet::new();
        let mut queue = VecDeque::new();
        for idx in 0..coords.len() {
            if !all_points.contains(&idx) {
                queue.push_back(idx);
                break;
            }
        }
        while let Some(current) = queue.pop_front() {
            if all_points.contains(&current) {
                continue;
            }
            tried.insert(current);
            all_points.insert(current);
            for idx in 0..coords.len() {
                if !tried.contains(&idx) && !all_points.contains(&idx) {
                    if coords[idx].distance(&coords[current]) <= 3 {
                        queue.push_back(idx);
                    }
                }
            }
        }
        if tried.is_empty() {
            break;
        }
        total_constellations += 1;
    }
    total_constellations
}

#[cfg(test)]
mod tests {
    use crate::day25::*;

    #[test]
    fn part1() {
        let input = input_generator("0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0");
        let result = solve_part1(&input);
        assert_eq!(result, 2);

        let input = input_generator("1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2");
        let result = solve_part1(&input);
        assert_eq!(result, 3);

        let input = input_generator("1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2");
        let result = solve_part1(&input);
        assert_eq!(result, 8);
    }
}

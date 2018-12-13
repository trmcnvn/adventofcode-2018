use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

pub struct Coord {
    x: usize,
    y: usize,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Coord> {
    input
        .trim()
        .lines()
        .map(|x| {
            let mut coords = x.split(", ").map(|y| y.parse::<usize>().unwrap());
            Coord {
                x: coords.next().unwrap(),
                y: coords.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(coords: &[Coord]) -> usize {
    let x_min_max = (
        coords.iter().map(|c| c.x).min().unwrap(),
        coords.iter().map(|c| c.x).max().unwrap(),
    );
    let y_min_max = (
        coords.iter().map(|c| c.y).min().unwrap(),
        coords.iter().map(|c| c.y).max().unwrap(),
    );
    let mut grid: HashMap<usize, usize> = HashMap::new();

    for x in x_min_max.0..=x_min_max.1 {
        for y in y_min_max.0..=y_min_max.1 {
            let mut min_distance: Option<usize> = None;
            let mut best_index: Option<usize> = None;

            for (idx, coord) in coords.iter().enumerate() {
                let distance = man_dist(coord.x, coord.y, x, y);
                if min_distance == None || distance < min_distance.unwrap() {
                    min_distance = Some(distance);
                    best_index = Some(idx);
                } else if distance == min_distance.unwrap() {
                    best_index = None;
                }
            }

            match best_index {
                Some(idx) => {
                    if x != x_min_max.0 && x != x_min_max.1 && y != y_min_max.0 && y != y_min_max.1
                    {
                        *grid.entry(idx).or_insert(0) += 1;
                    } else {
                        *grid.entry(idx).or_insert(0) = 0;
                    }
                }
                None => continue,
            }
        }
    }
    *grid.values().max().unwrap()
}

#[aoc(day6, part2)]
pub fn solve_part2(coords: &[Coord]) -> usize {
    part2_solver(coords, 10000)
}

fn part2_solver(coords: &[Coord], limit: usize) -> usize {
    let x_min_max = (
        coords.iter().map(|c| c.x).min().unwrap(),
        coords.iter().map(|c| c.x).max().unwrap(),
    );
    let y_min_max = (
        coords.iter().map(|c| c.y).min().unwrap(),
        coords.iter().map(|c| c.y).max().unwrap(),
    );

    let mut result: usize = 0;
    for x in x_min_max.0..=x_min_max.1 {
        for y in y_min_max.0..=y_min_max.1 {
            let mut distance_sum: usize = 0;
            for coord in coords {
                let distance = man_dist(coord.x, coord.y, x, y);
                distance_sum += distance;
            }
            if distance_sum < limit {
                result += 1;
            }
        }
    }
    result
}

fn man_dist(source_x: usize, source_y: usize, target_x: usize, target_y: usize) -> usize {
    let x: isize = source_x as isize - target_x as isize;
    let y: isize = source_y as isize - target_y as isize;
    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    #[test]
    fn part1() {
        let coords = input_generator(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
        );
        let result = solve_part1(&coords);
        assert_eq!(result, 17);
    }

    #[test]
    fn part2() {
        let coords = input_generator(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9",
        );
        let result = part2_solver(&coords, 32);
        assert_eq!(result, 16);
    }
}

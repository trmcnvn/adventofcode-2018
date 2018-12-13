use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Clone)]
pub struct Coord {
    x: isize,
    y: isize,
}

#[derive(Clone)]
pub struct Point {
    position: Coord,
    velocity: Coord,
}

impl Point {
    fn new(coords: &[isize]) -> Self {
        Self {
            position: Coord {
                x: coords[0],
                y: coords[1],
            },
            velocity: Coord {
                x: coords[2],
                y: coords[3],
            },
        }
    }

    fn step(&mut self, delta: isize) {
        self.position.x += delta * self.velocity.x;
        self.position.y += delta * self.velocity.y;
    }
}

fn get_bounds(points: &[Point]) -> (isize, isize, isize, isize) {
    let left = points.iter().map(|p| p.position.x).min().unwrap();
    let right = points.iter().map(|p| p.position.x).max().unwrap();
    let top = points.iter().map(|p| p.position.y).min().unwrap();
    let bottom = points.iter().map(|p| p.position.y).max().unwrap();
    (left, right, top, bottom)
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Point> {
    let re = Regex::new(r"position=<[ ]?(.+), [ ]?(.+)> velocity=<[ ]?(.+), [ ]?(.+)>").unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            Point::new(
                &cap.iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<isize>().unwrap())
                    .collect::<Vec<isize>>(),
            )
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(points: &[Point]) -> String {
    solve(points).0
}

#[aoc(day10, part2)]
pub fn solve_part2(points: &[Point]) -> usize {
    solve(points).1
}

fn solve(points: &[Point]) -> (String, usize) {
    let mut output = String::from("\n");

    let mut last_delta = 0;
    let mut last_points: Vec<Point> = Vec::new();

    for step in 0.. {
        let mut mut_points = points.to_vec();
        for point in &mut mut_points {
            point.step(step);
        }

        let (_, _, top, bottom) = get_bounds(&mut_points);
        let delta = bottom - top;

        if last_delta != 0 && delta > last_delta {
            let (left, right, top, bottom) = get_bounds(&last_points);
            let x = ((right - left) + 1) as usize;
            let y = ((bottom - top) + 1) as usize;

            let mut canvas = vec![vec![b' '; x]; y];
            for point in last_points {
                let y = (point.position.y - top) as usize;
                let x = (point.position.x - left) as usize;
                canvas[y][x] = b'#';
            }

            for row in canvas {
                for cell in row {
                    output.push(cell as char);
                }
                output.push('\n');
            }
            return (output, (step - 1) as usize);
        }

        last_delta = delta;
        last_points.clear();
        last_points.append(&mut mut_points);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn part1() {
        let input = input_generator(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>",
        );
        let result = solve_part1(&input);
        assert_eq!(result, "\n#   #  ###\n#   #   # \n#   #   # \n#####   # \n#   #   # \n#   #   # \n#   #   # \n#   #  ###\n");
    }

    #[test]
    fn part2() {
        let input = input_generator(
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 3);
    }
}

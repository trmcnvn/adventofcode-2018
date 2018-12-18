use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use regex::Regex;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone)]
struct Point(usize, usize);

#[derive(Clone)]
struct World {
    clay: HashSet<Point>,
    water: HashMap<Point, u8>,
    points_handled: HashSet<Point>,
}

impl World {
    fn drip(&mut self, point: &Point) {
        if self.points_handled.contains(point) {
            return;
        }
        self.points_handled.insert(point.clone());

        let y_max = self.clay.iter().max_by_key(|p| p.1).unwrap().1;
        let has_clay_below = (point.1..=y_max).find(|y| self.clay.contains(&Point(point.0, *y)));
        if has_clay_below.is_none() {
            for y in point.1..=y_max {
                self.water.insert(Point(point.0, y), b'|');
            }
            return;
        }

        let clay_y = has_clay_below.unwrap();
        for y in point.1..clay_y {
            self.water.insert(Point(point.0, y), b'|');
        }

        let mut current = clay_y - 1;
        loop {
            let (left_is_wall, left_x) = self.peek_at(&Point(point.0, current), -1);
            let (right_is_wall, right_x) = self.peek_at(&Point(point.0, current), 1);

            if left_is_wall && right_is_wall {
                for x in left_x + 1..right_x {
                    self.water.insert(Point(x, current), b'~');
                }
            } else {
                for x in left_x + 1..right_x {
                    self.water.insert(Point(x, current), b'|');
                }
                if !left_is_wall {
                    self.drip(&Point(left_x, current));
                }
                if !right_is_wall {
                    self.drip(&Point(right_x, current));
                }
                break;
            }
            current -= 1;
        }
    }

    fn peek_at(&self, point: &Point, change: isize) -> (bool, usize) {
        let mut current = Point((point.0 as isize + change) as usize, point.1);
        loop {
            if self.clay.contains(&current) {
                return (true, current.0);
            } else if !self.clay.contains(&Point(current.0, current.1 + 1)) {
                if self.water.contains_key(&Point(current.0, current.1 + 1))
                    && *self.water.get(&Point(current.0, current.1 + 1)).unwrap() == b'~'
                {
                    current = Point((current.0 as isize + change) as usize, current.1);
                    continue;
                }
                return (false, current.0);
            }
            current = Point((current.0 as isize + change) as usize, current.1);
        }
    }
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> World {
    let mut clay: HashSet<Point> = HashSet::new();
    let re = Regex::new(r"[xy]=(\d+),\s[x|y]=(\d+..\d+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps[1].parse::<usize>().unwrap();
        let b: Vec<usize> = caps[2]
            .split("..")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        match line.as_bytes()[0] {
            b'x' => {
                for y in b[0]..=b[1] {
                    clay.insert(Point(a, y));
                }
            }
            b'y' => {
                for x in b[0]..=b[1] {
                    clay.insert(Point(x, a));
                }
            }
            _ => unreachable!(),
        };
    }
    World {
        clay,
        water: HashMap::new(),
        points_handled: HashSet::new(),
    }
}

/// How many tiles can the water reach within the range of y values in your scan?
#[aoc(day17, part1)]
fn solve_part1(world: &World) -> usize {
    let y_min = world.clay.iter().min_by_key(|p| p.1).unwrap().1;
    let mut our_world = world.to_owned();
    our_world.drip(&Point(500, y_min));
    our_world.water.len()
}

/// How many water tiles are left after the water spring stops producing water and
/// all remaining water not at rest has drained?
#[aoc(day17, part2)]
fn solve_part2(world: &World) -> usize {
    let y_min = world.clay.iter().min_by_key(|p| p.1).unwrap().1;
    let mut our_world = world.to_owned();
    our_world.drip(&Point(500, y_min));
    our_world.water.iter().filter(|&(_, v)| *v == b'~').count()
}

#[cfg(test)]
mod tests {
    use crate::day17::*;

    #[test]
    fn part1() {
        let input = input_generator(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 57);
    }

    #[test]
    fn part2() {
        let input = input_generator(
            "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504",
        );
        let result = solve_part2(&input);
        assert_eq!(result, 29);
    }
}

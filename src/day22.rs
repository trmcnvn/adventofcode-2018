use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point(isize, isize);
impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let &Point(x, y) = self;
        vec![
            Point(x, y - 1),
            Point(x - 1, y),
            Point(x + 1, y),
            Point(x, y + 1),
        ]
    }
}

struct Region(usize, RegionType);

#[derive(Eq, PartialEq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum ToolType {
    Torch,
    Climbing,
    HandsFree,
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> (Point, HashMap<Point, Region>) {
    let mut lines = input.lines();
    let depth: usize = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let target: Vec<isize> = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    let (target_x, target_y) = (target[0], target[1]);

    let mut cave: HashMap<Point, Region> = HashMap::new();
    for y in 0..depth as isize {
        for x in 0..depth as isize {
            let geological_index = if (x == 0 && y == 0) || (x == target_x && y == target_y) {
                0
            } else if y == 0 {
                x as usize * 16807
            } else if x == 0 {
                y as usize * 48271
            } else {
                let a = cave.get(&Point(x - 1, y)).unwrap().0;
                let b = cave.get(&Point(x, y - 1)).unwrap().0;
                a * b
            };
            let erosion_level = (geological_index + depth) % 20183;
            let region_type = match erosion_level % 3 {
                0 => RegionType::Rocky,
                1 => RegionType::Wet,
                _ => RegionType::Narrow,
            };
            cave.insert(Point(x, y), Region(erosion_level, region_type));
        }
    }
    (Point(target_x, target_y), cave)
}

/// What is the total risk level for the smallest rectangle that includes 0,0
/// and the target's coordinates?
#[aoc(day22, part1)]
fn solve_part1((target, cave): &(Point, HashMap<Point, Region>)) -> usize {
    let mut risk_level = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            let region = cave.get(&Point(x, y)).unwrap();
            risk_level += match region.1 {
                RegionType::Rocky => 0,
                RegionType::Wet => 1,
                RegionType::Narrow => 2,
            };
        }
    }
    risk_level
}

/// What is the fewest number of minutes you can take to reach the target?
///
/// TODO: Dijktra this...
#[aoc(day22, part2)]
fn solve_part2((target, cave): &(Point, HashMap<Point, Region>)) -> usize {
    let mut queue: VecDeque<(Point, ToolType, usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(Point, ToolType)> = HashSet::new();
    queue.push_back((Point(0, 0), ToolType::Torch, 0, 0));
    visited.insert((Point(0, 0), ToolType::Torch));

    while let Some((point, tool, switching, time)) = queue.pop_front() {
        if switching > 0 {
            if switching != 1 || visited.insert((point.clone(), tool.clone())) {
                queue.push_back((point.clone(), tool.clone(), switching - 1, time + 1));
            }
            continue;
        }

        if point == *target && tool == ToolType::Torch {
            return time;
        }

        let region_type = &cave.get(&point).unwrap().1;
        for neighbour in point.neighbours() {
            if neighbour.0 < 0 || neighbour.1 < 0 {
                continue;
            }
            match cave.get(&neighbour).unwrap().1 {
                RegionType::Rocky => {
                    if tool != ToolType::HandsFree
                        && visited.insert((neighbour.clone(), tool.clone()))
                    {
                        queue.push_back((neighbour.clone(), tool.clone(), 0, time + 1));
                    }
                }
                RegionType::Wet => {
                    if tool != ToolType::Torch && visited.insert((neighbour.clone(), tool.clone()))
                    {
                        queue.push_back((neighbour.clone(), tool.clone(), 0, time + 1));
                    }
                }
                RegionType::Narrow => {
                    if tool != ToolType::Climbing
                        && visited.insert((neighbour.clone(), tool.clone()))
                    {
                        queue.push_back((neighbour.clone(), tool.clone(), 0, time + 1));
                    }
                }
            };
        }

        match region_type {
            RegionType::Rocky => {
                queue.push_back((point.clone(), ToolType::Climbing, 6, time + 1));
                queue.push_back((point.clone(), ToolType::Torch, 6, time + 1));
            }
            RegionType::Wet => {
                queue.push_back((point.clone(), ToolType::Climbing, 6, time + 1));
                queue.push_back((point.clone(), ToolType::HandsFree, 6, time + 1));
            }
            RegionType::Narrow => {
                queue.push_back((point.clone(), ToolType::HandsFree, 6, time + 1));
                queue.push_back((point.clone(), ToolType::Torch, 6, time + 1));
            }
        };
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::day22::*;

    #[test]
    fn part1() {
        let input = input_generator("depth: 510\ntarget: 10,10");
        let result = solve_part1(&input);
        assert_eq!(result, 114);
    }

    #[test]
    fn part2() {
        let input = input_generator("depth: 510\ntarget: 10,10");
        let result = solve_part2(&input);
        assert_eq!(result, 45);
    }
}

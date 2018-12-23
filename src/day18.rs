use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

#[derive(Clone, Eq, PartialEq)]
enum Tile {
    Ground,
    Tree,
    Lumberyard,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Point(isize, isize);
impl Point {
    fn adjacents(&self) -> Vec<Point> {
        let Point(x, y) = *self;
        vec![
            Point(x - 1, y - 1),
            Point(x, y - 1),
            Point(x + 1, y - 1),
            Point(x - 1, y),
            Point(x + 1, y),
            Point(x - 1, y + 1),
            Point(x, y + 1),
            Point(x + 1, y + 1),
        ]
    }
}

type Acres = HashMap<Point, Tile>;

fn mutate_acres(current: &Acres, mutable: &mut Acres) {
    for (key, value) in current {
        match value {
            Tile::Ground => {
                let mut tree_count = 0;
                for point in key.adjacents() {
                    if let Some(tile) = current.get(&point) {
                        if *tile == Tile::Tree {
                            tree_count += 1;
                        }
                    }
                }
                if tree_count >= 3 {
                    mutable.insert(key.clone(), Tile::Tree);
                }
            }
            Tile::Tree => {
                let mut lumber_count = 0;
                for point in key.adjacents() {
                    if let Some(tile) = current.get(&point) {
                        if *tile == Tile::Lumberyard {
                            lumber_count += 1;
                        }
                    }
                }
                if lumber_count >= 3 {
                    mutable.insert(key.clone(), Tile::Lumberyard);
                }
            }
            Tile::Lumberyard => {
                let mut lumber_count = 0;
                let mut tree_count = 0;
                for point in key.adjacents() {
                    if let Some(tile) = current.get(&point) {
                        match tile {
                            Tile::Tree => tree_count += 1,
                            Tile::Lumberyard => lumber_count += 1,
                            _ => {}
                        };
                    }
                }
                if lumber_count == 0 || tree_count == 0 {
                    mutable.insert(key.clone(), Tile::Ground);
                }
            }
        }
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Acres {
    let mut acres: Acres = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.as_bytes().iter().enumerate() {
            let tile = match tile {
                b'.' => Tile::Ground,
                b'|' => Tile::Tree,
                b'#' => Tile::Lumberyard,
                _ => unreachable!(),
            };
            acres.insert(Point(x as isize, y as isize), tile);
        }
    }
    acres
}

/// What will the total resource value of the lumber collection area be after 10 minutes?
#[aoc(day18, part1)]
fn solve_part1(acres: &Acres) -> usize {
    let mut mutable_acres = acres.to_owned();
    for _ in 0..10 {
        let current_acres = mutable_acres.to_owned();
        mutate_acres(&current_acres, &mut mutable_acres);
    }

    let wooded_acres = mutable_acres
        .iter()
        .filter(|&(_, v)| *v == Tile::Tree)
        .count();
    let lumber_acres = mutable_acres
        .iter()
        .filter(|&(_, v)| *v == Tile::Lumberyard)
        .count();
    wooded_acres * lumber_acres
}

/// What will the total resource value of the lumber collection area be after 1000000000 minutes?
#[aoc(day18, part2)]
fn solve_part2(acres: &Acres) -> usize {
    let mut pattern_start = 0;
    let mut pattern_end = 0;
    let mut patterns: HashMap<String, usize> = HashMap::new();

    let mut mutable_acres = acres.to_owned();
    for n in 0..1_000_000_000 {
        let current_acres = mutable_acres.to_owned();
        mutate_acres(&current_acres, &mut mutable_acres);
        let grid_id: String = mutable_acres
            .values()
            .map(|t| match t {
                Tile::Ground => '.',
                Tile::Tree => '|',
                Tile::Lumberyard => '#',
            })
            .collect();
        if patterns.contains_key(&grid_id) {
            pattern_start = *patterns.get(&grid_id).unwrap();
            pattern_end = n + 1;
            break;
        } else {
            patterns.insert(grid_id, n + 1);
        }
    }

    let mut mutable_acres = acres.to_owned();
    for _ in 0..(pattern_start + ((1_000_000_000 - pattern_start) % (pattern_end - pattern_start)))
    {
        let current_acres = mutable_acres.to_owned();
        mutate_acres(&current_acres, &mut mutable_acres);
    }
    let wooded_acres = mutable_acres
        .iter()
        .filter(|&(_, v)| *v == Tile::Tree)
        .count();
    let lumber_acres = mutable_acres
        .iter()
        .filter(|&(_, v)| *v == Tile::Lumberyard)
        .count();
    wooded_acres * lumber_acres
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    fn part1() {
        let input = input_generator(
            ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 1147);
    }
}

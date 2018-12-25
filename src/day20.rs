/// I originally tried to implement a parser approach using Pest.rs to explore that library.
/// This is an implementation by a redditor from r/adventofcode.
///
/// Hoping to return and redo a solution using Pest or another parser.
use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use pathfinding::prelude::dijkstra_all;
use std::collections::{BTreeMap, BTreeSet};

type Point = (isize, isize);

#[aoc_generator(day20)]
fn input_generator(input: &str) -> HashMap<Point, (Point, isize)> {
    let mut map = BTreeMap::new();
    explore(&mut map, (0, 0), input.as_bytes(), &mut 1);
    dijkstra_all(&(0, 0), |pos| {
        map.get(pos)
            .into_iter()
            .flat_map(|neighbours| neighbours.iter().map(|n| (*n, 1)))
    })
    .into_iter()
    .collect()
}

#[aoc(day20, part1)]
fn solve_part1(cells: &HashMap<Point, (Point, isize)>) -> isize {
    cells.values().map(|(_, c)| *c).max().unwrap()
}

#[aoc(day20, part2)]
fn solve_part2(cells: &HashMap<Point, (Point, isize)>) -> usize {
    cells.values().filter(|&(_, c)| *c >= 1000).count()
}

fn explore(
    map: &mut BTreeMap<Point, BTreeSet<Point>>,
    start: Point,
    input: &[u8],
    index: &mut usize,
) -> Vec<Point> {
    let mut exits = vec![start];
    loop {
        match input[*index] {
            b'|' | b')' | b'$' => return exits,
            b'(' => {
                let mut new_exits = BTreeSet::new();
                while input[*index] != b')' {
                    let old_index = *index;
                    new_exits.extend(exits.iter().flat_map(|pos| {
                        *index = old_index + 1;
                        explore(map, *pos, input, index)
                    }));
                }
                exits = new_exits.into_iter().collect();
            }
            dir => {
                let dir = usize::from((dir ^ (dir >> 2)) & 3);
                let (dx, dy) = ([1, 0, -1, 0][dir], [0, -1, 0, 1][dir]);
                for pos in &mut exits {
                    let newpos = (pos.0 + dx, pos.1 + dy);
                    map.entry(*pos).or_insert_with(BTreeSet::new).insert(newpos);
                    *pos = newpos;
                }
            }
        }
        *index += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::day20::*;

    #[test]
    fn part1() {
        let input = input_generator("^ENWWW(NEEE|SSE(EE|N))$");
        let result = solve_part1(&input);
        assert_eq!(result, 10);

        let input = input_generator("^WNE$");
        let result = solve_part1(&input);
        assert_eq!(result, 3);

        let input = input_generator("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        let result = solve_part1(&input);
        assert_eq!(result, 18);
    }
}

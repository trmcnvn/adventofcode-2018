use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use std::collections::VecDeque;

#[derive(Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Default)]
struct Game {
    units: HashMap<Point, usize>,
    world: HashMap<Point, u8>,
}

impl Game {
    fn attack_unit_at_point(&mut self, point: &Point, attack: usize) -> bool {
        let unit_health = self.units.get_mut(point).unwrap();
        if *unit_health <= attack {
            self.units.remove_entry(point);
            self.world.insert(point.clone(), b'.');
            true
        } else {
            *unit_health -= attack;
            false
        }
    }

    fn find_direct_target(&self, point: &Point) -> Option<Point> {
        let mut lowest_health = std::usize::MAX;
        let mut target: Option<Point> = None;
        let source_tile = self.world.get(point).unwrap();
        for position in point.neighbours() {
            if !self.units.contains_key(&position) {
                continue;
            }
            let target_tile = self.world.get(&position).unwrap();
            if source_tile == target_tile {
                continue; // Not an enemy
            }
            let health = self.units.get(&position).unwrap();
            if target.is_none() || *health < lowest_health {
                lowest_health = *health;
                target = Some(position);
            }
        }
        target
    }

    fn find_next_point(&self, source: &Point) -> Option<Point> {
        let units: Vec<Point> = self.units.keys().cloned().filter(|k| k != source).collect();
        if units.is_empty() {
            return None;
        }

        // get all the points where we want to end up on
        let mut goals = HashSet::new();
        let source_tile = self.world.get(source).unwrap();
        let enemies = units
            .into_iter()
            .filter(|k| self.world.get(k).unwrap() != source_tile);
        for enemy in enemies {
            for neighbour in enemy.neighbours() {
                if self.world.contains_key(&neighbour)
                    && *self.world.get(&neighbour).unwrap() == b'.'
                {
                    goals.insert(neighbour.clone());
                }
            }
        }

        // traverse outwards from our source point
        let mut queue = VecDeque::new();
        let mut parents: HashMap<Point, Point> = HashMap::new();
        queue.push_back(source.clone());
        parents.insert(source.clone(), Point(-1, -1));
        while let Some(point) = queue.pop_front() {
            for neighbour in point.neighbours() {
                if parents.contains_key(&neighbour)
                    || (self.world.contains_key(&neighbour)
                        && *self.world.get(&neighbour).unwrap() != b'.')
                {
                    continue;
                }
                queue.push_back(neighbour.clone());
                parents.insert(neighbour.clone(), point.clone());
            }
        }

        // find paths that lead to our goals
        let mut possible_paths: Vec<Vec<Point>> = goals
            .iter()
            .filter(|k| parents.contains_key(k))
            .map(|k| {
                let mut steps = Vec::new();
                let mut point = k.clone();
                while point != *source {
                    steps.push(point.clone());
                    point = parents.get(&point).unwrap().clone();
                }
                steps.reverse();
                steps
            })
            .collect();
        if possible_paths.is_empty() {
            return None;
        }

        // grab the best path by reading order
        possible_paths.sort_by_key(|p| p.len());
        let min_length = possible_paths[0].len();
        let mut best_paths: Vec<Vec<Point>> = possible_paths
            .into_iter()
            .filter(|p| p.len() == min_length)
            .collect();
        best_paths.sort_by_key(|p| Point(p.last().unwrap().1, p.last().unwrap().0));
        let best_path = best_paths[0].clone();
        Some(best_path[0].clone())
    }

    fn move_from(&mut self, source: &Point, target: &Point) {
        let (_, health) = self.units.remove_entry(source).unwrap();
        let unit_type = self.world.get(source).unwrap();
        self.world.insert(target.clone(), *unit_type);
        self.world.insert(source.clone(), b'.');
        self.units.insert(target.clone(), health);
    }
}

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Game {
    let mut state = Game::default();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.as_bytes().iter().enumerate() {
            let tile = match symbol {
                b'G' | b'E' => {
                    state.units.insert(Point(x as isize, y as isize), 200);
                    symbol
                }
                _ => symbol,
            };
            state.world.insert(Point(x as isize, y as isize), *tile);
        }
    }
    state
}

#[aoc(day15, part1)]
fn solve_part1(state: &Game) -> usize {
    let mut round_count = 0;
    let mut state = state.to_owned();
    loop {
        let mut dead_positions: HashSet<Point> = HashSet::new();
        let mut keys: Vec<Point> = state.units.keys().cloned().collect();
        keys.sort_by_key(|point| Point(point.1, point.0));
        for point in keys.clone().into_iter() {
            let unit = state.units.get_mut(&point);
            // Unit was killed in previous iteration
            if unit.is_none() || dead_positions.contains(&point) {
                continue;
            }

            // Can we attack?
            let target_position = state.find_direct_target(&point);
            if let Some(position) = target_position {
                if state.attack_unit_at_point(&position, 3) {
                    dead_positions.insert(position);
                }
                continue;
            }

            // We didn't attack, so can we move?
            if let Some(position) = state.find_next_point(&point) {
                state.move_from(&point, &position);
                // We can attack after moving...
                let target_position = state.find_direct_target(&position);
                if let Some(position) = target_position {
                    if state.attack_unit_at_point(&position, 3) {
                        dead_positions.insert(position);
                    }
                }
            } else {
                let source_tile = state.world.get(&point).unwrap();
                let enemy_units: Vec<Point> = state
                    .units
                    .keys()
                    .cloned()
                    .filter(|k| state.world.get(k).unwrap() != source_tile)
                    .collect();
                if !enemy_units.is_empty() {
                    continue;
                }
                return round_count * state.units.values().sum::<usize>();
            }
        }
        round_count += 1;
    }
}

#[aoc(day15, part2)]
fn solve_part2(game_state: &Game) -> usize {
    let mut round_count = 0;
    let mut elf_attack = 4;
    loop {
        let mut state = game_state.to_owned();
        'inner: loop {
            let mut dead_positions: HashSet<Point> = HashSet::new();
            let mut keys: Vec<Point> = state.units.keys().cloned().collect();
            keys.sort_by_key(|point| Point(point.1, point.0));
            for point in keys.clone().into_iter() {
                let unit = state.units.get_mut(&point);
                // Unit was killed in previous iteration
                if unit.is_none() || dead_positions.contains(&point) {
                    continue;
                }

                let attack = match state.world.get(&point).unwrap() {
                    b'E' => elf_attack,
                    _ => 3,
                };

                // Can we attack?
                let target_position = state.find_direct_target(&point);
                if let Some(position) = target_position {
                    let is_enemy_elf = *state.world.get(&position).unwrap() == b'E';
                    if state.attack_unit_at_point(&position, attack) {
                        dead_positions.insert(position);
                        if is_enemy_elf {
                            break 'inner;
                        }
                    }
                    continue;
                }

                // We didn't attack, so can we move?
                if let Some(position) = state.find_next_point(&point) {
                    state.move_from(&point, &position);
                    // We can attack after moving...
                    let target_position = state.find_direct_target(&position);
                    if let Some(position) = target_position {
                        let is_enemy_elf = *state.world.get(&position).unwrap() == b'E';
                        if state.attack_unit_at_point(&position, attack) {
                            dead_positions.insert(position);
                            if is_enemy_elf {
                                break 'inner;
                            }
                        }
                    }
                } else {
                    let goblins: Vec<Point> = state
                        .units
                        .keys()
                        .cloned()
                        .filter(|k| *state.world.get(k).unwrap() == b'G')
                        .collect();
                    if !goblins.is_empty() {
                        continue;
                    }
                    return round_count * state.units.values().sum::<usize>();
                }
            }
            round_count += 1;
        }
        round_count = 0;
        elf_attack += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::*;

    #[test]
    fn part1() {
        let input = input_generator(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        );
        let result = solve_part1(&input);
        assert_eq!(result, 27730);
    }

    #[test]
    fn part2() {
        let mut input = input_generator(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        );
        let mut result = solve_part2(&input);
        assert_eq!(result, 4988);

        input = input_generator(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
        );
        result = solve_part2(&input);
        assert_eq!(result, 31284);

        input = input_generator(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
        );
        result = solve_part2(&input);
        assert_eq!(result, 3478);

        input = input_generator(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
        );
        result = solve_part2(&input);
        assert_eq!(result, 6474);

        input = input_generator(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
        );
        result = solve_part2(&input);
        assert_eq!(result, 1140);
    }
}

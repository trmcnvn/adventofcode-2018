use aoc_runner_derive::aoc;

const GRID_SIZE: usize = 300;

fn get_power(x: usize, y: usize, serial: usize) -> isize {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial;
    power *= rack_id;
    power = (power / 100) % 10;
    power as isize - 5
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> String {
    let serial: usize = input.parse().unwrap();
    let size = 3;
    let mut grid = [[0_isize; GRID_SIZE]; GRID_SIZE];
    for (x, x_val) in grid.iter_mut().enumerate().take(GRID_SIZE) {
        for (y, y_val) in x_val.iter_mut().enumerate().take(GRID_SIZE) {
            *y_val = get_power(x, y, serial);
        }
    }

    let mut best_power = 0;
    let mut result = (0, 0);
    for x in 0..(GRID_SIZE - size) {
        for y in 0..(GRID_SIZE - size) {
            let power = grid.iter().skip(x).take(size).fold(0_isize, |acc, row| {
                acc + row.iter().skip(y).take(size).sum::<isize>()
            });
            if power > best_power {
                best_power = power;
                result = (x, y);
            }
        }
    }
    format!("{},{}", result.0, result.1)
}

/// https://en.wikipedia.org/wiki/Summed-area_table
#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> String {
    let serial: usize = input.parse().unwrap();
    let mut grid = [[0_isize; GRID_SIZE]; GRID_SIZE];
    for (x, x_val) in grid.iter_mut().enumerate().take(GRID_SIZE) {
        for (y, y_val) in x_val.iter_mut().enumerate().take(GRID_SIZE) {
            *y_val = get_power(x, y, serial);
        }
    }

    for x in 0..GRID_SIZE {
        let mut sum = 0;
        for y in 0..GRID_SIZE {
            sum = sum + grid[x][y];
            if x == 0 {
                grid[x][y] = sum
            } else {
                grid[x][y] = grid[x - 1][y] + sum
            }
        }
    }

    let mut best_sum = 0;
    let mut result = (0, 0, 0);
    for x in 1..GRID_SIZE {
        for y in 1..GRID_SIZE {
            let min_size = (GRID_SIZE - (x + 1)).min(GRID_SIZE - (y + 1));
            for size in 1..=min_size {
                let sum = grid[x + size - 1][y + size - 1] + grid[x - 1][y - 1]
                    - grid[x - 1][y + size - 1]
                    - grid[x + size - 1][y - 1];
                if sum > best_sum {
                    best_sum = sum;
                    result = (x, y, size);
                }
            }
        }
    }
    format!("{},{},{}", result.0, result.1, result.2)
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn part1() {
        let mut result = solve_part1("42");
        assert_eq!(result, "21,61");
    }

    #[test]
    fn part2() {
        let result = solve_part2("18");
        assert_eq!(result, "90,269,16");
    }
}

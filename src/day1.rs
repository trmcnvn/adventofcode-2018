use hashbrown::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut frequencies = HashSet::new();

    for value in input.iter().cycle() {
        frequency += value;
        if !frequencies.insert(frequency) {
            break;
        }
    }

    frequency
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    fn part1() {
        let test_input = input_generator(
            r"+1
-2
+3
+1",
        );
        let result = solve_part1(&test_input);
        assert_eq!(result, 3);
    }

    #[test]
    fn part2() {
        let test_input = input_generator(
            r"+1
-2
+3
+1",
        );
        let result = solve_part2(&test_input);
        assert_eq!(result, 2);
    }
}

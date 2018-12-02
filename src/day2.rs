use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let (mut doubles, mut triples) = (0, 0);
    for id in input.lines() {
        let mut map = HashMap::new();
        for c in id.chars() {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }

        if map.values().any(|&x| x == 2) {
            doubles += 1;
        }
        if map.values().any(|&x| x == 3) {
            triples += 1;
        }
    }
    doubles * triples
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> String {
    let ids: Vec<&str> = input.lines().collect();
    for (idx, current) in ids.iter().enumerate() {
        for next in ids.iter().skip(idx + 1) {
            if is_single_distance(current, next) {
                return current
                    .chars()
                    .zip(next.chars())
                    .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
                    .collect();
            }
        }
    }
    unreachable!();
}

fn is_single_distance(first: &str, second: &str) -> bool {
    if first.len() != second.len() {
        return false;
    }

    let mut difference_found = false;
    for (c1, c2) in first.chars().zip(second.chars()) {
        if c1 != c2 {
            if difference_found {
                return false;
            }
            difference_found = true;
        }
    }

    true
}

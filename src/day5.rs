#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(chars: &[char]) -> usize {
    react_polymer(chars).len()
}

#[aoc(day5, part2)]
pub fn solve_part2(chars: &[char]) -> usize {
    let polymer = react_polymer(chars);
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let w: Vec<_> = alphabet
        .iter()
        .map(|c| {
            let x: String = polymer.iter().collect();
            let u: char = c.to_ascii_uppercase();
            let y: Vec<char> = x
                .chars()
                .filter_map(|x| match x {
                    _ if x == *c => None,
                    _ if x == u => None,
                    _ => Some(x),
                })
                .collect();
            react_polymer(&y).len()
        })
        .collect();
    *w.iter().min().unwrap()
}

fn react_polymer(chars: &[char]) -> Vec<char> {
    let mut slice: Vec<char> = Vec::new();
    for c in chars {
        slice.push(*c);
        if slice.len() < 2 {
            continue;
        }
        let x = slice[slice.len() - 2];
        if x.eq_ignore_ascii_case(&c)
            && ((x.is_ascii_uppercase() && c.is_ascii_lowercase())
                || (x.is_ascii_lowercase() && c.is_ascii_uppercase()))
        {
            slice.remove(slice.len() - 2);
            slice.remove(slice.len() - 1);
        }
    }
    slice
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn part1() {
        let test_input = input_generator("dabAcCaCBAcCcaDA");
        let result = solve_part1(&test_input);
        assert_eq!(result, 10);
    }

    #[test]
    fn part2() {
        let test_input = input_generator("dabAcCaCBAcCcaDA");
        let result = solve_part2(&test_input);
        assert_eq!(result, 4);
    }
}

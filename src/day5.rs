#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(chars: &Vec<char>) -> usize {
    react_polymer(chars)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<char>) -> usize {
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut min_size: usize = 0;
    for letter in alphabet {
        let remaining: Vec<_> = input.clone().into_iter().filter(|c| !c.eq_ignore_ascii_case(&letter)).collect();
        let size = react_polymer(&remaining);
        if min_size == 0 || size < min_size { min_size = size; }
    }
    min_size
}

fn react_polymer(input: &Vec<char>) -> usize {
    let mut chars = input.clone();
    let func = |a: char, b: char| -> bool {
        if a.eq_ignore_ascii_case(&b) {
            if a.is_ascii_uppercase() && b.is_ascii_lowercase() { return true; }
            if a.is_ascii_lowercase() && b.is_ascii_uppercase() { return true; }
        }
        false
    };
    let mut last_size = 0;
    loop {
        let mut i = 0;
        while i < chars.len() {
            if i == 0 { i += 1; continue; }
            let last_char = chars[i - 1];
            if func(chars[i], last_char) {
                chars.remove(i);
                chars.remove(i - 1);
            } else {
                i += 1;
            }
        }
        if chars.len() == last_size { break; }
        last_size = chars.len();
    }
    chars.len()
}

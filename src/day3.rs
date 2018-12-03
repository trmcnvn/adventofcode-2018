use regex::Regex;
use std::collections::HashMap;

const CLAIM_PATTERN: &str = r"(?x)
    (?P<id>\d+)
    \s@\s
    (?P<x>\d+),(?P<y>\d+):
    \s
    (?P<w>\d+)x(?P<h>\d+)";

pub struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let re = Regex::new(CLAIM_PATTERN).unwrap();
    input
        .lines()
        .map(|s| {
            let matches = re.captures(s).unwrap();
            Claim {
                id: matches["id"].parse::<i32>().unwrap(),
                x: matches["x"].parse::<i32>().unwrap(),
                y: matches["y"].parse::<i32>().unwrap(),
                w: matches["w"].parse::<i32>().unwrap(),
                h: matches["h"].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

// Build map of [x + w][y + h] = <# of overlaps>
fn build_map(claims: &[Claim]) -> HashMap<i32, HashMap<i32, Vec<i32>>> {
    let mut map: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
    for claim in claims {
        for w in 0..claim.w {
            for h in 0..claim.h {
                let x = map
                    .entry(claim.x + w)
                    .or_insert_with(HashMap::<i32, Vec<i32>>::new);
                let y = x.entry(claim.y + h).or_insert_with(Vec::new);
                y.push(claim.id);
            }
        }
    }
    map
}

#[aoc(day3, part1)]
pub fn solve_part1(claims: &[Claim]) -> i32 {
    build_map(claims).values().fold(0, |a, b| {
        a + b.values().filter(|x| x.len() > 1).count() as i32
    })
}

#[aoc(day3, part2)]
pub fn solve_part2(claims: &[Claim]) -> i32 {
    let map = build_map(claims);
    let all_ids: Vec<i32> = claims.iter().map(|x| x.id).collect();
    let overlapping_ids: Vec<i32> = map
        .values()
        .flat_map(|x| x.values().cloned().filter(|y| y.len() > 1))
        .flatten()
        .collect();

    for id in all_ids {
        if !overlapping_ids.contains(&id) {
            return id;
        }
    }

    unreachable!();
}

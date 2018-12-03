use regex::Regex;
use std::collections::HashMap;

pub struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|s| {
            let matches = re.captures(s).unwrap();
            Claim {
                id: matches[1].parse::<i32>().unwrap(),
                x: matches[2].parse::<i32>().unwrap(),
                y: matches[3].parse::<i32>().unwrap(),
                w: matches[4].parse::<i32>().unwrap(),
                h: matches[5].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

// { (x + w): { (y + h): [ids, ...] } }
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
    let mut overlapping_ids: Vec<i32> = map
        .values()
        .flat_map(|x| x.values().cloned().filter(|y| y.len() > 1))
        .flatten()
        .collect();
    overlapping_ids.sort();
    overlapping_ids.dedup();

    for id in all_ids {
        if !overlapping_ids.contains(&id) {
            return id;
        }
    }

    unreachable!();
}

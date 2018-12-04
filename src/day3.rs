use hashbrown::{HashMap, HashSet};
use regex::Regex;

pub struct Claim {
    id: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|s| {
            let matches = re.captures(s).unwrap();
            Claim {
                id: matches[1].parse().unwrap(),
                x: matches[2].parse().unwrap(),
                y: matches[3].parse().unwrap(),
                w: matches[4].parse().unwrap(),
                h: matches[5].parse().unwrap(),
            }
        })
        .collect()
}

fn build_map(claims: &[Claim]) -> HashMap<(u32, u32), Vec<u32>> {
    let mut map: HashMap<_, Vec<u32>> = HashMap::new();
    for claim in claims {
        for i in claim.x..claim.x + claim.w {
            for j in claim.y..claim.y + claim.h {
                map.entry((i, j)).or_default().push(claim.id);
            }
        }
    }
    map
}

#[aoc(day3, part1)]
pub fn solve_part1(claims: &[Claim]) -> u32 {
    build_map(claims).values().filter(|x| x.len() > 1).count() as u32
}

#[aoc(day3, part2)]
pub fn solve_part2(claims: &[Claim]) -> u32 {
    let all_ids: HashSet<_> = claims.iter().map(|x| x.id).collect();
    let other_ids: HashSet<_> = build_map(claims)
        .values()
        .cloned()
        .filter(|x| x.len() > 1)
        .flatten()
        .collect();

    for id in all_ids {
        if !other_ids.contains(&id) {
            return id;
        }
    }
    unreachable!();
}

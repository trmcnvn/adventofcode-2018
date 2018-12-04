use hashbrown::HashMap;
use regex::Regex;

// can't return HashMap from aoc_generator...?
fn input_generator(input: &str) -> HashMap<u32, HashMap<u32, u32>> {
    // We need to sort the input into the correct order by date.
    let mut sorted_input: Vec<&str> = input.lines().collect();
    sorted_input.sort();

    let re = Regex::new(r"\d+:(\d+)\]\s(\w+)\s#?(\d+)?").unwrap();

    // Store log as { guard_id: { minute: count } }
    let mut activities: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let (mut last_guard_id, mut last_sleeping_minute) = (0, 0);

    for line in sorted_input {
        let matches = re.captures(line).unwrap();
        let minute = matches[1].parse().unwrap();
        match &matches[2] {
            "Guard" => {
                last_guard_id = matches[3].parse().unwrap();
            },
            "falls" => {
                last_sleeping_minute = minute;
            },
            "wakes" => {
                for minute in last_sleeping_minute..minute {
                    let entry = activities.entry(last_guard_id).or_default();
                    *entry.entry(minute).or_default() += 1;
                }
            },
            _ => panic!("uhhh....") // shouldn't happen? bad input
        }
    }
    activities
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let activities = input_generator(&input);
    let (guard_id, minutes) = activities.iter().max_by_key(|(_, v)| {
        v.values().sum::<u32>()
    }).unwrap();
    let (minute, _) = minutes.iter().max_by_key(|&(_, &v)| v).unwrap();
    guard_id * minute
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let activities = input_generator(&input);
    let (guard_id, minutes) = activities.iter().max_by_key(|(_k, v)| {
        v.values().max()
    }).unwrap();
    let (minute, _) = minutes.iter().max_by_key(|&(_, &v)| v).unwrap();
    guard_id * minute
}

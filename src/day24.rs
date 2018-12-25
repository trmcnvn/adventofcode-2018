use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<units>\d+) units each with (?P<health>\d+) hit points ?(?:\((?:(?P<type_a>weak|immune) to (?P<types_a>[^;)]+))?(?:;? ?(?:(?P<type_b>weak|immune) to (?P<types_b>[^;)]+)))?\))? with an attack that does (?P<attack>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)").unwrap();
}

#[derive(Hash, Clone, Copy, Eq, PartialEq)]
enum AttackType {
    Radiation,
    Cold,
    Fire,
    Bludgeoning,
    Slashing,
}

impl AttackType {
    fn from_str(input: &str) -> Self {
        match input {
            "radiation" => AttackType::Radiation,
            "cold" => AttackType::Cold,
            "fire" => AttackType::Fire,
            "bludgeoning" => AttackType::Bludgeoning,
            "slashing" => AttackType::Slashing,
            _ => unreachable!(),
        }
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
enum ArmyType {
    ImmuneSystem,
    Infection,
}

#[derive(Hash, Clone, Eq, PartialEq)]
struct Group {
    army_type: ArmyType,
    unit_count: usize,
    health_points: usize,
    attack_power: usize,
    attack_type: AttackType,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    initiative: usize,
    target_id: Option<usize>,
}

impl Group {
    fn build(input: &str, army_type: ArmyType) -> Self {
        let caps = RE.captures(input).unwrap();
        let mut weaknesses = Vec::new();
        let mut immunities = Vec::new();

        let types_a = caps.name("types_a");
        if let Some(_types) = types_a {
            for _type in _types.as_str().split(", ") {
                let actual_type = AttackType::from_str(_type);
                if &caps["type_a"] == "weak" {
                    weaknesses.push(actual_type);
                } else {
                    immunities.push(actual_type);
                }
            }
        }

        let types_b = caps.name("types_b");
        if let Some(_types) = types_b {
            for _type in _types.as_str().split(", ") {
                let actual_type = AttackType::from_str(_type);
                if &caps["type_b"] == "weak" {
                    weaknesses.push(actual_type);
                } else {
                    immunities.push(actual_type);
                }
            }
        }

        Group {
            army_type,
            unit_count: caps["units"].parse().unwrap(),
            health_points: caps["health"].parse().unwrap(),
            attack_power: caps["attack"].parse().unwrap(),
            attack_type: AttackType::from_str(&caps["attack_type"]),
            weaknesses,
            immunities,
            initiative: caps["initiative"].parse().unwrap(),
            target_id: None,
        }
    }

    fn effective_power(&self, boost: usize) -> usize {
        let ap = if self.army_type == ArmyType::ImmuneSystem {
            self.attack_power + boost
        } else {
            self.attack_power
        };
        self.unit_count * ap
    }

    fn total_damage(&self, effective_power: usize, attack_type: AttackType) -> usize {
        if self.is_weak_to(attack_type) {
            effective_power * 2
        } else if self.is_immune_to(attack_type) {
            0
        } else {
            effective_power
        }
    }

    fn is_weak_to(&self, attack_type: AttackType) -> bool {
        self.weaknesses.contains(&attack_type)
    }

    fn is_immune_to(&self, attack_type: AttackType) -> bool {
        self.immunities.contains(&attack_type)
    }
}

#[aoc_generator(day24)]
fn input_generator(input: &str) -> Vec<Group> {
    let mut armies: Vec<Group> = Vec::new();

    let immune_iter = input.lines().skip(1).take_while(|line| !line.is_empty());
    for group in immune_iter.clone() {
        armies.push(Group::build(group, ArmyType::ImmuneSystem));
    }

    let infections_iter = input.lines().skip(immune_iter.count() + 3);
    for group in infections_iter {
        armies.push(Group::build(group, ArmyType::Infection));
    }

    armies
}

/// As it stands now, how many units would the winning army have?
#[aoc(day24, part1)]
fn solve_part1(armies: &[Group]) -> usize {
    let mut armies = armies.to_owned();
    solve(&mut armies, 0);
    armies.iter().map(|g| g.unit_count).sum()
}

/// How many units does the immune system have left after getting the smallest boost it needs to win?
#[aoc(day24, part2)]
fn solve_part2(_armies: &[Group]) -> usize {
    let mut boost_value = 0;
    loop {
        let mut armies = _armies.to_owned();
        solve(&mut armies, boost_value);
        if armies
            .iter()
            .filter(|g| g.army_type == ArmyType::Infection && g.unit_count > 0)
            .count()
            == 0
        {
            return armies
                .iter()
                .filter(|g| g.army_type == ArmyType::ImmuneSystem)
                .map(|g| g.unit_count)
                .sum();
        }
        boost_value += 1;
    }
}

fn solve(armies: &mut [Group], boost: usize) {
    loop {
        let mut attackers = (0..armies.len()).collect::<Vec<_>>();
        attackers.sort_unstable_by_key(|&i| {
            (
                Reverse(armies[i].effective_power(boost)),
                Reverse(armies[i].initiative),
            )
        });

        // Targeting
        let mut attacker_targets = vec![None; attackers.len()];
        for idx in attackers {
            let attacker = &armies[idx];
            attacker_targets[idx] = armies
                .iter()
                .enumerate()
                .filter(|(i, g)| {
                    g.army_type != attacker.army_type
                        && g.unit_count > 0
                        && !attacker_targets.contains(&Some(*i))
                })
                .max_by_key(|(_, g)| {
                    (
                        g.total_damage(attacker.effective_power(boost), attacker.attack_type),
                        g.effective_power(boost),
                        g.initiative,
                    )
                })
                .map(|(i, _)| i);
        }

        // Attacking
        let mut attackers = (0..armies.len()).collect::<Vec<_>>();
        attackers.sort_unstable_by_key(|&i| Reverse(armies[i].initiative));
        let units_before_attack = armies.iter().map(|g| g.unit_count).sum::<usize>();
        for idx in attackers {
            let attacker = &armies[idx];
            if attacker.unit_count == 0 {
                continue;
            }

            if let Some(target_idx) = attacker_targets[idx] {
                let mut real_damage = armies[target_idx]
                    .total_damage(attacker.effective_power(boost), attacker.attack_type);
                if real_damage < armies[target_idx].health_points {
                    continue;
                }
                while armies[target_idx].unit_count > 0 {
                    real_damage -= armies[target_idx].health_points;
                    armies[target_idx].unit_count -= 1;
                    if real_damage < armies[target_idx].health_points {
                        break;
                    }
                }
            }
        }
        let units_after_attack = armies.iter().map(|g| g.unit_count).sum::<usize>();
        if units_before_attack == units_after_attack {
            break;
        }

        // Finished?
        if armies
            .iter()
            .filter(|g| g.army_type == ArmyType::ImmuneSystem && g.unit_count > 0)
            .count()
            == 0
            || armies
                .iter()
                .filter(|g| g.army_type == ArmyType::Infection && g.unit_count > 0)
                .count()
                == 0
        {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day24::*;

    #[test]
    fn part1() {
        let input = input_generator("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4");
        let result = solve_part1(&input);
        assert_eq!(result, 5216);
    }

    #[test]
    fn part2() {
        let input = input_generator("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 6077 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 1595 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4");
        let result = solve_part2(&input);
        assert_eq!(result, 51);
    }
}

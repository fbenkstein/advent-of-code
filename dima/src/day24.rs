use std::cmp::Ordering;
use std::collections::HashSet;
use text_io::{scan, try_scan};

#[derive(Debug, PartialEq, Eq)]
enum Army {
    ImmuneSystem,
    Infection,
}

impl Default for Army {
    fn default() -> Self {
        Army::ImmuneSystem
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Group {
    army: Army,
    units: u32,
    hit_points: u32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    attack_damage: u32,
    attack_type: String,
    initiative: u32,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_damage
    }

    fn damage_to(&self, other: &Self) -> u32 {
        assert!(self.army != other.army);
        if other.immunities.contains(&self.attack_type) {
            0
        } else {
            if other.weaknesses.contains(&self.attack_type) {
                2 * self.effective_power()
            } else {
                self.effective_power()
            }
        }
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.effective_power()
            .cmp(&other.effective_power())
            .reverse()
            .then(self.initiative.cmp(&other.initiative))
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str, army: Army) -> Group {
    let mut group = Group {
        army,
        ..Group::default()
    };

    let attrs_pos = line.find('(').and_then(|start| {
        line[start + 1..]
            .find(')')
            .map(|end| (start, start + 1 + end + 1))
    });

    if let Some((attrs_start, attrs_end)) = attrs_pos {
        scan!((&line[..attrs_start]).bytes() =>
            "{} units each with {} hit points", group.units, group.hit_points);
        scan!((&line[attrs_end..]).bytes() =>
            " with an attack that does {} {} damage at initiative {}",
            group.attack_damage, group.attack_type, group.initiative);

        for attr in (&line[attrs_start + 1..attrs_end - 1]).split("; ") {
            let mut words = attr.splitn(3, ' ');
            let v = match words.next().unwrap() {
                "weak" => &mut group.weaknesses,
                "immune" => &mut group.immunities,
                _ => panic!("wrong input"),
            };
            v.extend(words.skip(1).next().unwrap().split(", ").map(String::from));
        }
    } else {
        scan!(line.bytes() =>
            "{} units each with {} hit points with an attack that does {} {} damage at initiative {}",
            group.units, group.hit_points, group.attack_damage, group.attack_type, group.initiative);
    }

    group
}

fn parse(input: &str) -> Vec<Group> {
    let mut parts = input.split("\n\n");
    let immune_system = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| parse_line(l, Army::ImmuneSystem));
    let infection = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| parse_line(l, Army::Infection));
    immune_system.chain(infection).collect()
}

fn target_selection(groups: &mut [Group]) -> Vec<Option<usize>> {
    groups.sort();

    let mut imm_targets: Vec<usize> = groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.army == Army::Infection)
        .map(|(idx, _)| idx)
        .collect();
    let mut inf_targets: Vec<usize> = groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.army == Army::ImmuneSystem)
        .map(|(idx, _)| idx)
        .collect();

    groups
        .iter()
        .map(|attacking| {
            let targets = if attacking.army == Army::ImmuneSystem {
                &mut imm_targets
            } else {
                &mut inf_targets
            };
            let target_idx = targets
                .iter()
                .enumerate()
                .filter_map(|(idx, defending_idx)| {
                    let defending = &groups[*defending_idx];
                    let damage = attacking.damage_to(defending);
                    if damage == 0 {
                        None
                    } else {
                        Some((
                            (damage, defending.effective_power(), defending.initiative),
                            idx,
                        ))
                    }
                })
                .max()
                .map(|(_, idx)| idx);
            if let Some(idx) = target_idx {
                let defending_idx = Some(targets[idx]);
                targets.swap_remove(idx);
                defending_idx
            } else {
                None
            }
        })
        .collect()
}

fn attacking(groups: &mut Vec<Group>, targets: &[Option<usize>]) {
    let mut order: Vec<usize> = (0..groups.len()).collect();
    order.sort_by_key(|idx| {
        let g = &groups[*idx];
        -(g.initiative as i32)
    });
    for idx in order {
        if let Some(target_idx) = targets[idx] {
            let attacking = &groups[idx];
            let defending = &groups[target_idx];
            let damage = attacking.damage_to(defending);
            let killed = damage / defending.hit_points;
            let new_units = defending.units.saturating_sub(killed);
            groups[target_idx].units = new_units;
        }
    }

    groups.retain(|g| g.units > 0);
}

pub fn solve(input: &str) -> u32 {
    let mut groups = parse(input);
    while groups.iter().any(|g| g.army == Army::ImmuneSystem)
        && groups.iter().any(|g| g.army == Army::Infection)
    {
        let targets = target_selection(&mut groups);
        attacking(&mut groups, &targets[..]);
    }
    groups.iter().map(|g| g.units).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_selection() {
        const INPUT: &str = r#"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"#;
        let mut groups = parse(INPUT);
        println!("{:?}", groups);
        let targets = target_selection(&mut groups);
        println!("{:?}", targets);
        for (idx, g) in groups.iter().enumerate() {
            println!(
                "{:?} -> {:?}",
                g.units,
                targets[idx].map(|i| groups[i].units)
            );
        }
        let p1 = solve(INPUT);
        assert_eq!(p1, 5216);
    }

    // 4485 -> 989
    // 989 -> 801
    // 17 -> 4485
    // 801 -> 17
}

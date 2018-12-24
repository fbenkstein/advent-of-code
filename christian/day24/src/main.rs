#[macro_use]
extern crate text_io;

use itertools::Itertools;

use std::io::{self, prelude::*};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Group {
    id: usize,
    initiative: isize,
    count: isize,
    hp: isize,
    attack: isize,
    attack_type: String,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Input {
    system: Vec<Group>,
    infection: Vec<Group>,
}

fn parse(input: &Vec<String>) -> Input {
    let groups = |result: &mut Vec<Group>, iter: &mut std::slice::Iter<String>| {
        while let Some(line) = iter.next() {
            if line.is_empty() {
                break;
            }
            let mut group: Group = Default::default();
            group.id = result.len() + 1;
            let traits: String;
            scan!(line.bytes() => "{} units each with {} hit points ({}) with an attack that does {} {} damage at initiative {}",
              group.count, group.hp, traits, group.attack, group.attack_type, group.initiative);
            for list in traits.split("; ") {
                if list.starts_with("weak to ") {
                    group.weaknesses = list["weak to ".len()..]
                        .split(", ")
                        .map(|x| x.into())
                        .collect();
                } else {
                    assert!(list.starts_with("immune to "));
                    group.immunities = list["immune to ".len()..]
                        .split(", ")
                        .map(|x| x.into())
                        .collect();
                }
            }
            // println!("{}\n{:?}", line, group);
            result.push(group);
        }
    };
    let mut result: Input = Default::default();
    let mut iter = input.iter();
    assert_eq!(iter.next().unwrap(), "Immune System:");
    groups(&mut result.system, &mut iter);
    assert_eq!(iter.next().unwrap(), "Infection:");
    groups(&mut result.infection, &mut iter);

    result
}

fn damage(from: &Group, to: &Group) -> isize {
    if to.immunities.contains(&from.attack_type) {
        return 0;
    }
    if to.weaknesses.contains(&from.attack_type) {
        return from.count * from.attack * 2;
    }
    from.count * from.attack
}

fn targets(from: &Vec<Group>, to: &Vec<Group>) -> Vec<Option<usize>> {
    let order = from
        .iter()
        .enumerate()
        .sorted_by_key(|(_, x)| (x.attack * x.count, x.initiative));
    let mut used = vec![false; to.len()];
    order
        .iter()
        .rev()
        .map(|(i, x)| {
            if let Some(best) = to
                .iter()
                .enumerate()
                .filter_map(|(i, y)| {
                    if !used[i] && damage(x, y) > 0 {
                        Some((damage(x, y), y.count * y.attack, y.initiative, i))
                    } else {
                        None
                    }
                })
                .max()
            {
                used[best.3] = true;
                (i, Some(best.3))
            } else {
                (i, None)
            }
        })
        .sorted_by_key(|x| x.0)
        .iter()
        .map(|x| x.1)
        .collect()

    /*
        During the target selection phase, each group attempts to choose one target.
        In decreasing order of effective power, groups choose their targets;
        in a tie, the group with the higher initiative chooses first.
        The attacking group chooses to target the group in the enemy army to which it would deal the most damage
        (after accounting for weaknesses and immunities,
        but not accounting for whether the defending group has enough units to actually receive all of that damage).

    If an attacking group is considering two defending groups to which it would deal equal damage,
    it chooses to target the defending group with the largest effective power;
    if there is still a tie, it chooses the defending group with the highest initiative
    If it cannot deal any defending groups damage, it does not choose a target.
    Defending groups can only be chosen as a target by one attacking group.
        */
}

fn combat(mut input: Input, verbose: bool, boost: isize) -> bool {
    for x in input.system.iter_mut() {
        x.attack += boost;
    }
    let mut count = 0;
    while !input.system.is_empty() && !input.infection.is_empty() && count < 10000 {
        if verbose {
            println!("\nImmune System:");
            for x in &input.system {
                println!("Group {} contains {} units", x.id, x.count);
            }
            println!("Infection System:");
            for x in &input.infection {
                println!("Group {} contains {} units", x.id, x.count);
            }
        }

        let system_choice = targets(&input.system, &input.infection);
        let infection_choice = targets(&input.infection, &input.system);

        if verbose {
            for (i, x) in system_choice.iter().enumerate() {
                if let Some(target) = x {
                    println!(
                        "System group {} will attack {}",
                        input.system[i].id, input.infection[*target].id
                    );
                }
            }

            for (i, x) in infection_choice.iter().enumerate() {
                if let Some(target) = x {
                    println!(
                        "Infection group {} will attack {}",
                        input.infection[i].id, input.system[*target].id
                    );
                }
            }
        }

        let order = input
            .system
            .iter()
            .enumerate()
            .map(|(i, x)| (0, i, x.initiative))
            .chain(
                input
                    .infection
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (1, i, x.initiative)),
            )
            .sorted_by_key(|x| x.2);
        for (class, i, _) in order.iter().rev() {
            if *class == 0 {
                let x = &input.system[*i];
                if x.count <= 0 {
                    continue;
                }
                if let Some(target) = system_choice[*i] {
                    if verbose {
                        println!(
                            "System group {} attacks {}, killing {} units",
                            x.id,
                            input.infection[target].id,
                            damage(&x, &input.infection[target]) / input.infection[target].hp
                        );
                    }
                    assert!(damage(&x, &input.infection[target]) > 0);
                    input.infection[target].count -=
                        damage(&x, &input.infection[target]) / input.infection[target].hp;
                }
            } else {
                let x = &input.infection[*i];
                if x.count <= 0 {
                    continue;
                }
                if let Some(target) = infection_choice[*i] {
                    if verbose {
                        println!(
                            "Infection group {} attacks {}, killing {} units",
                            x.id,
                            input.system[target].id,
                            damage(&x, &input.system[target]) / input.system[target].hp
                        );
                    }
                    assert!(damage(&x, &input.system[target]) > 0);
                    input.system[target].count -=
                        damage(&x, &input.system[target]) / input.system[target].hp;
                }
            }
        }
        input.system.retain(|x| x.count > 0);
        input.infection.retain(|x| x.count > 0);
        count += 1;
    }
    println!(
        "System units left: {}",
        input.system.iter().map(|x| x.count).sum::<isize>()
    );
    if verbose {
        for x in input.system.iter() {
            println!("   {:?}", x);
        }
    }
    println!(
        "Infection units left: {}",
        input.infection.iter().map(|x| x.count).sum::<isize>()
    );
    if verbose {
        for x in input.infection.iter() {
            println!("   {:?}", x);
        }
    }
    !input.system.is_empty() && input.infection.is_empty()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    let mut boost = 0;
    while !combat(input.clone(), false, boost) {
        boost += 1;
    }
}

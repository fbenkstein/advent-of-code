mod parser;

use std::fs::File;
use std::io::prelude::*;

use std::collections::VecDeque;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let (_, parsed_input) =
        parser::parse_input(parser::CompleteStr(&contents)).expect("failed to parse input");
    let (mut pots, rules) = parsed_input;
    let mut next_pots = VecDeque::new();

    let mut old_sum = 0;
    let mut old_delta = 0;
    let mut stability_counter = 0;

    let mut global_offset = 0;
    const GENERATIONS: isize = 50000000000;
    for i in 0..GENERATIONS as isize {
        next_pots.clear();
        next_pots.resize(pots.len(), false);
        let mut offset = 0;
        for pot_id in -2..(pots.len() as isize) + 2 {
            let pot_id = pot_id as isize;
            if let Some(rule_that_apply) = rules.iter().find(|rule| {
                rule.pattern
                    .iter()
                    .zip(pot_id - 2..pot_id + 3)
                    .all(|(&pattern_element, pos)| {
                        if pos < 0 || pos >= pots.len() as isize {
                            return pattern_element == false;
                        }
                        pattern_element == pots[pos as usize]
                    })
            }) {
                if (pot_id < 0 && !rule_that_apply.has_plant)
                    || (pot_id >= pots.len() as isize && !rule_that_apply.has_plant)
                {
                    continue;
                }
                // we grow the culture when we are out of bounds
                for _ in pot_id..0 {
                    next_pots.push_front(false);
                    offset += 1;
                }

                for _ in (pots.len() as isize)..pot_id + 1 {
                    next_pots.push_back(false);
                }
                next_pots[(pot_id + offset) as usize] = rule_that_apply.has_plant;
            }
        }

        global_offset += offset;

        let sum = pots
            .iter()
            .enumerate()
            .filter_map(|(i, pot)| {
                if !pot {
                    None
                } else {
                    Some(i as isize - global_offset)
                }
            })
            .sum::<isize>();

        let delta = (sum - old_sum);
        if old_delta == delta {
            stability_counter += 1;
        }
        if stability_counter > 5 {
            println!("THE FUCKING ANSWER IS {}", (GENERATIONS - i) * delta + sum);
            return;
        }

        old_delta = sum - old_sum;
        old_sum = sum;

        let gen1 = next_pots
            .iter()
            .map(|&p| if p { '#' } else { '.' })
            .collect::<String>();
        println!("{}", gen1);

        std::mem::swap(&mut pots, &mut next_pots);
    }
}

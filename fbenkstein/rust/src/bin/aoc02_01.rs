extern crate itertools;

use itertools::Itertools;

fn main() {
    let data = include_str!("input_02.txt");
    let mut count_twos = 0;
    let mut count_threes = 0;

    for line in data.lines() {
        let sorted_chars = line.chars().sorted();
        let grouped_chars = sorted_chars.iter().group_by(|x| *x);
        let group_lengths = grouped_chars.into_iter().map(|(_, g)| g.count());
        let mut found_twos = false;
        let mut found_threes = false;
        group_lengths
            .take_while(|l| {
                found_twos = found_twos || *l == 2;
                found_threes = found_threes || *l == 3;
                !(found_twos && found_threes)
            })
            .for_each(drop);
        if found_twos {
            count_twos += 1;
        }
        if found_threes {
            count_threes += 1;
        }
    }

    println!("{}", count_twos * count_threes);
}

extern crate itertools;

use itertools::Itertools;

fn main() {
    let data = include_str!("input_02.txt");

    for (id0, id1) in data.lines().cartesian_product(data.lines()) {
        let reduced: String = id0
            .chars()
            .zip(id1.chars())
            .filter_map(|(c0, c1)| if c0 == c1 { Some(c0) } else { None })
            .collect();
        let diff_count = id0.len() - reduced.len();

        if diff_count == 1 {
            println!("{}", reduced);
            break;
        }
    }
}

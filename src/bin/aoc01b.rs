use std::collections::BTreeSet;

fn main() {
    let data = include_str!("input_a.txt");
    let mut current_freq = 0;
    let mut seen_freqs = BTreeSet::new();
    seen_freqs.insert(current_freq);

    let input = std::iter::repeat(data).map(|s| s.lines()).flatten().map(|s| {
        let sign = s.get(0..1).and_then(|s| match s {
            "+" => Some(1),
            "-" => Some(-1),
            _ => None,
        }).unwrap();
        let value: i32 = s.get(1..).unwrap().parse().unwrap();
        let change = sign * value;
        change
    });

    for change in input {
        current_freq += change;

        if !seen_freqs.insert(current_freq) {
            println!("{}", current_freq);
            break;
        }
    }
}

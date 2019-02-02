fn main() {
    let data = include_str!("input_01.txt");
    let mut sum = 0;

    for line in data.lines() {
        let sign = line
            .get(0..1)
            .and_then(|s| match s {
                "+" => Some(1),
                "-" => Some(-1),
                _ => None,
            })
            .unwrap();
        let value: i32 = line.get(1..).unwrap().parse().unwrap();
        sum += sign * value;
    }

    println!("{}", sum);
}

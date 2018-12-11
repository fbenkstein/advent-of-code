#[macro_use]
extern crate text_io;

fn solve1(list: &String) -> (usize, usize) {
    let explodes = |a: char, b: char| a != b && a.to_ascii_uppercase() == b.to_ascii_uppercase();

    let chars: Vec<_> = list.chars().collect();
    let mut destroyed = vec![false; chars.len()];
    for pos in 0..chars.len() {
        if let Some(x) = destroyed[..pos].iter().rev().position(|&x| !x) {
            if explodes(chars[pos - 1 - x], chars[pos]) {
                destroyed[pos - 1 - x] = true;
                destroyed[pos] = true;
            }
        }
    }
    (destroyed.iter().filter(|&&x| !x).count(), chars.len())
}

fn solve2(list: &String) {
    let best = (b'a'..=b'z')
        .map(|c| {
            let mut x = list.clone();
            x.retain(|other| other.to_ascii_lowercase() != c as char);
            (solve1(&x), c)
        })
        .min()
        .unwrap();
    println!("Best to remove {} : {:?}", best.1 as char, (best.0));
}

fn main() {
    let list = read!("{}");
    let (num_left, len) = solve1(&list);
    println!("Num left: {} / {}", num_left, len);
    solve2(&list);
}

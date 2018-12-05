#[allow(dead_code)]
pub fn solve_checksum(input: &str) -> usize {
    let contains_n_let = |line: &str, n: usize| {
        char_iter::new('a', 'z')
            .filter(|c| line.chars().filter(|x| x == c).count() == n)
            .count()
            > 0
    };
    input.lines().filter(|line| contains_n_let(line, 2)).count()
        * input.lines().filter(|line| contains_n_let(line, 3)).count()
}

pub fn solve(input: &str) -> String {
    for (a, b) in iproduct!(input.lines(), input.lines()) {
        let m: String = a
            .chars()
            .zip(b.chars())
            .filter(|(a, b)| a == b)
            .map(|(a, _b)| a)
            .collect();
        if m.len() + 1 == a.len() {
            return m;
        }
    }
    String::new()
}

fn compactor<I: Iterator<Item = char>>(input: I) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    for c in input {
        let pop = stack
            .last()
            .map(|prev| {
                prev.to_ascii_uppercase() == c.to_ascii_uppercase()
                    && prev.is_uppercase() != c.is_uppercase()
            })
            .unwrap_or(false);
        if pop {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack
}

pub fn solve2(input: &str) -> Option<usize> {
    let mut units: Vec<_> = input.chars().map(|c| c.to_ascii_lowercase()).collect();
    units.sort();
    units.dedup();
    units
        .into_iter()
        .map(|unit| compactor(input.chars().filter(|c| c.to_ascii_lowercase() != unit)).len())
        .min()
}

pub fn solve(input: &str) -> (usize, Option<usize>) {
    (compactor(input.chars()).len(), solve2(input))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(compactor("dabAcCaCBAcCcaDA".chars()).len(), 10);
    }
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.split(" ").map(|s| s.parse().unwrap())
}

fn checksum(input: &mut Iterator<Item = usize>) -> usize {
    let len_children = input.next().unwrap();
    let len_metadata = input.next().unwrap();
    let checksum: usize = (0..len_children).map(|_| checksum(input)).sum();
    let checksum = checksum + input.take(len_metadata).sum::<usize>();
    checksum
}

fn value(input: &mut Iterator<Item = usize>) -> usize {
    let len_children = input.next().unwrap();
    let len_metadata = input.next().unwrap();
    let values: Vec<usize> = (0..len_children).map(|_| value(input)).collect();
    let metadata = input.take(len_metadata);
    if values.is_empty() {
        metadata.sum()
    } else {
        metadata.filter_map(|idx| values.get(idx - 1)).sum()
    }
}

pub fn solve(input: &str) -> (usize, usize) {
    (checksum(&mut parse(input)), value(&mut parse(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(&mut parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")),
            138
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(value(&mut parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2")), 66);
    }
}

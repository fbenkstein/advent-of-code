fn parse_claim(input: &str) -> std::option::Option<(usize, usize, usize, usize, usize)> {
    if let (Some(no), Some(left), Some(top), Some(width), Some(height)) = scan_fmt!(
        input,
        "#{d} @ {d},{d}: {d}x{d}",
        usize,
        usize,
        usize,
        usize,
        usize
    ) {
        Some((no, left, top, width, height))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn solve_part1(input: &str) -> usize {
    let mut claim_map = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        if let Some((_, left, top, width, height)) = parse_claim(line) {
            for x in left..left + width {
                for y in top..top + height {
                    claim_map[x][y] += 1;
                }
            }
        }
    }
    claim_map
        .iter()
        .map(|x| x.iter())
        .flatten()
        .filter(|x| **x > 1)
        .count()
}

pub fn solve(input: &str) -> Option<usize> {
    let mut claim_map = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        if let Some((_, left, top, width, height)) = parse_claim(line) {
            for x in left..left + width {
                for y in top..top + height {
                    claim_map[x][y] += 1;
                }
            }
        }
    }
    for line in input.lines() {
        if let Some((no, left, top, width, height)) = parse_claim(line) {
            let mut conflict = false;
            for x in left..left + width {
                for y in top..top + height {
                    if claim_map[x][y] > 1 {
                        conflict = true;
                    }
                }
            }
            if !conflict {
                return Some(no);
            }
        }
    }
    None
}

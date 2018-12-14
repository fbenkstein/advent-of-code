use std::error::Error;
use std::str;

fn score(num_recipies: usize) -> usize {
    let mut board: Vec<u8> = Vec::with_capacity(num_recipies + 10);
    board.push(3);
    board.push(7);
    let mut i = 0;
    let mut j = 1;

    loop {
        let sum = board[i] + board[j];
        let digits = sum.to_string();
        let digits = digits.chars().map(|d| d.to_digit(10).unwrap() as u8);
        board.extend(digits);

        i = (i + board[i] as usize + 1) % board.len();
        j = (j + board[j] as usize + 1) % board.len();

        if num_recipies + 10 <= board.len() {
            return (0..10).fold(0, |acc, i| 10 * acc + board[num_recipies + i] as usize);
        }
    }
}

fn num_recipies(score: &str) -> usize {
    let mut board: Vec<u8> = vec![3, 7];
    let mut i = 0;
    let mut j = 1;

    let score: Vec<u8> = score
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut score_start = 0;

    loop {
        let sum = board[i] + board[j];
        let digits = sum.to_string();
        let digits = digits.chars().map(|d| d.to_digit(10).unwrap() as u8);
        board.extend(digits);

        i = (i + board[i] as usize + 1) % board.len();
        j = (j + board[j] as usize + 1) % board.len();

        while score_start + score.len() <= board.len() {
            let pattern = &board[score_start..score_start + score.len()];
            if pattern == &score[..] {
                return score_start;
            }
            score_start += 1;
        }
    }
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<Error>> {
    let n: usize = str::parse(input)?;
    Ok((score(n), num_recipies(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score(9), 5158916779);
        assert_eq!(score(5), 0124515891);
        assert_eq!(score(18), 9251071085);
        assert_eq!(score(2018), 5941429882);
    }

    #[test]
    fn test_num_recipies() {
        assert_eq!(num_recipies("51589"), 9);
        assert_eq!(num_recipies("01245"), 5);
        assert_eq!(num_recipies("92510"), 18);
        assert_eq!(num_recipies("59414"), 2018);
    }
}

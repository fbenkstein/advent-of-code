use std::error::Error;
use text_io::try_scan;

fn parse(input: &str) -> Result<(usize, usize), Box<Error>> {
    let num_players: usize;
    let num_marbles: usize;
    try_scan!(input.bytes() => "{} players; last marble is worth {} points", num_players, num_marbles);
    Ok((num_players, num_marbles))
}

fn highest_score(num_players: usize, num_marbles: usize) -> usize {
    struct PlacedMarble {
        prev: usize,
        next: usize,
        value: usize,
    };

    let mut scores = vec![0; num_players];
    let mut storage = Vec::with_capacity(num_marbles);
    storage.push(PlacedMarble {
        prev: 0,
        next: 0,
        value: 0,
    });

    let mut cur_marble_index = 0;
    let mut cur_player = 0;
    for marble in 1..=num_marbles {
        if marble % 23 != 0 {
            let new_marble_index = storage.len();

            // advance +2
            let next_marble_index = storage[storage[cur_marble_index].next].next;
            let next_marble = &mut storage[next_marble_index];
            let prev_marble_index = next_marble.prev;

            // add new marble before next marble
            next_marble.prev = new_marble_index;
            let prev_marble = &mut storage[prev_marble_index];
            prev_marble.next = new_marble_index;

            let new_marble = PlacedMarble {
                prev: prev_marble_index,
                next: next_marble_index,
                value: marble,
            };
            storage.push(new_marble);

            cur_marble_index = new_marble_index;
        } else {
            // advance -7
            let rem_marble_index = cur_marble_index;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;
            let rem_marble_index = storage[rem_marble_index].prev;

            let rem_marble = &storage[rem_marble_index];
            scores[cur_player] += marble + rem_marble.value;

            // remove marble
            let rem_marble_prev = rem_marble.prev;
            let rem_marble_next = rem_marble.next;
            let prev_marble = &mut storage[rem_marble_prev];
            prev_marble.next = rem_marble_next;
            let next_marble = &mut storage[rem_marble_next];
            next_marble.prev = rem_marble_prev;

            cur_marble_index = rem_marble_next;
        }
        cur_player = (cur_player + 1) % num_players;
    }
    scores.into_iter().max().unwrap()
}

pub fn solve(input: &str) -> Result<(usize, usize), Box<Error>> {
    let (num_players, num_marbles) = parse(input)?;
    let part_1 = highest_score(num_players, num_marbles);
    let part_2 = highest_score(num_players, 100 * num_marbles);
    Ok((part_1, part_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highest_score() -> Result<(), Box<Error>> {
        const INPUTS: [&str; 6] = [
            "9 players; last marble is worth 25 points",
            "10 players; last marble is worth 1618 points",
            "13 players; last marble is worth 7999 points",
            "17 players; last marble is worth 1104 points",
            "21 players; last marble is worth 6111 points",
            "30 players; last marble is worth 5807 points",
        ];

        const HIGHEST_SCORE: [usize; 6] = [32, 8317, 146373, 2764, 54718, 37305];

        for (&input, &score) in INPUTS.into_iter().zip(HIGHEST_SCORE.into_iter()) {
            let (num_players, num_marbles) = parse(input)?;
            assert_eq!(highest_score(num_players, num_marbles), score);
        }
        Ok(())
    }
}

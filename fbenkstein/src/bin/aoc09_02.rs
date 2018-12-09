extern crate regex;

use regex::Regex;

#[allow(dead_code)]
fn print_status(player: u32, current_marble_index: usize, marbles: &Vec<u32>, scores: &Vec<u32>) {
    println!(
        "[{}]: marbles: {}, high score: {}",
        if marbles.len() > 1 {
            player.to_string()
        } else {
            "-".to_string()
        },
        marbles
            .iter()
            .enumerate()
            .map(|(index, marble)| if index == current_marble_index {
                format!("({})", marble)
            } else {
                format!("{}", marble)
            })
            .collect::<Vec<_>>()
            .join(" "),
        scores
            .iter()
            .enumerate()
            .max_by_key(|(_index, score)| *score)
            .map(|(index, score)| format!("{} by player {}", score, index + 1))
            .unwrap(),
    );
}

fn get_high_score(player_count: u32, last_marble_score: u32) -> u32 {
    let remaining_marbles = 1..=last_marble_score;
    let mut marbles = vec![0];
    marbles.reserve(last_marble_score as usize + 1);
    let mut current_marble_index = 0;
    let mut scores = vec![0; player_count as usize];

    for (player, next_marble) in (0..player_count).cycle().zip(remaining_marbles) {
        // print_status(player, current_marble_index, &marbles, &scores);
        print!(
            "{} / {} = {:.3}\r",
            next_marble,
            last_marble_score,
            next_marble as f64 / last_marble_score as f64
        );

        if next_marble % 23 != 0 {
            current_marble_index = ((current_marble_index + 1) % marbles.len()) + 1;
            marbles.insert(current_marble_index, next_marble);
        } else {
            current_marble_index = (current_marble_index + marbles.len() - 7) % marbles.len();
            let removed_marble = marbles.remove(current_marble_index);
            let marble_score = removed_marble + next_marble;
            scores[player as usize] += marble_score;
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let inputs = [
        // "9 players; last marble is worth 0 points; high score is 32",
        "10 players; last marble is worth 1618 points: high score is 8317",
        "13 players; last marble is worth 7999 points: high score is 146373",
        "17 players; last marble is worth 1104 points: high score is 2764",
        "21 players; last marble is worth 6111 points: high score is 54718",
        "30 players; last marble is worth 5807 points: high score is 37305",
        "424 players; last marble is worth 71482 points",
        "424 players; last marble is worth 7148200 points",
    ]
    .iter()
    .map(|s| {
        let re =
            Regex::new(r"(\d+) players; last marble is worth (\d+) points").expect("invalid regex");
        let captures = re.captures(s).expect("string doesn't match");
        let player_count: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let last_marble_score: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        (player_count, last_marble_score)
    });

    for (player_count, last_marble_score) in inputs {
        let high_score = get_high_score(player_count, last_marble_score);
        println!(
            "{} players: last marble is worth {} points: high score is {}",
            player_count, last_marble_score, high_score
        );
    }
}

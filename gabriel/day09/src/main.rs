use std::collections::VecDeque;

pub trait VecDequeExt<T> {
    fn rotate(&mut self, n: isize);
}

// shameless crappy impl. of https://docs.python.org/3/library/collections.html#collections.deque.rotate
impl<T: Copy> VecDequeExt<T> for VecDeque<T> {
    fn rotate(&mut self, n: isize) {
        if n > 0 {
            (0..n).for_each(|_| {
                let m = self.pop_back().expect("empty list?");
                self.push_front(m);
            });
        } else {
            (n..0).for_each(|_| {
                let m = self.pop_front().expect("empty list?");
                self.push_back(m);
            });
        }
    }
}

fn max_score(players: usize, last_marble: usize) -> usize {
    let mut marbles: VecDeque<usize> = VecDeque::with_capacity(last_marble + 1);
    marbles.push_back(0);
    let mut scores = vec![0; players];
    for marble in 1..last_marble + 1 {
        if marble % 23 == 0 {
            marbles.rotate(7);
            scores[marble % players] += marble + marbles.pop_back().expect("no marbles to pop?");
            marbles.rotate(-1);
        } else {
            marbles.rotate(-1);
            marbles.push_back(marble);
        }
    }

    *scores.iter().max().expect("no maximum score, uh.")
}

fn main() {
    println!("Part 1 max. score: {}", max_score(478, 71240));
    println!("Part 2 max. score: {}", max_score(478, 71240 * 100));
}

#[test]
fn test_max_score() {
    assert_eq!(max_score(9, 25), 32);
    assert_eq!(max_score(10, 1618), 8317);
    assert_eq!(max_score(13, 7999), 146373);
    assert_eq!(max_score(17, 1104), 2764);
    assert_eq!(max_score(21, 6111), 54718);
    assert_eq!(max_score(30, 5807), 37305);
}

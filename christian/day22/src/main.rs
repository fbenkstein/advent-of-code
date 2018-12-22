#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use priority_queue::PriorityQueue;
use revord::RevOrd;
use std::collections::hash_map;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tool {
    Neither,
    Torch,
    Climbing,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct State {
    tool: Tool,
    pos: (usize, usize),
}

impl State {
    fn new(tool: Tool, pos: (usize, usize)) -> State {
        State { tool, pos }
    }
}

fn print(iter: impl Iterator<Item = usize>) -> Vec<u8> {
    let mut result = Vec::new();
    for x in iter {
        match x {
            0 => result.push(b'.'),
            1 => result.push(b'='),
            2 => result.push(b'|'),
            _ => panic!("Arghs"),
        }
    }
    result
}

fn to_strings(input: Vec<Vec<u8>>) -> Vec<String> {
    input
        .into_iter()
        .map(|vec| String::from_utf8(vec).unwrap())
        .collect()
}

fn generate(target: (usize, usize), size: (usize, usize), depth: usize) -> Vec<Vec<u8>> {
    let mut slice: Vec<usize> = (0..=size.0).map(|x| (x * 16807 + depth) % 20183).collect();
    let mut next = vec![0; size.0 + 1];
    let eval = |v: &usize| v % 3;
    print(slice.iter().map(eval));
    let mut sum: usize = slice.iter().map(eval).sum();
    let mut result = vec![print(slice.iter().map(eval))];
    for y in 1..=size.1 {
        next[0] = (y * 48271 + depth) % 20183;
        for x in 1..=size.0 {
            if (x, y) == target {
                next[x] = depth % 20183;
            } else {
                next[x] = (next[x - 1] * slice[x] + depth) % 20183;
            }
        }
        std::mem::swap(&mut next, &mut slice);
        sum += slice.iter().map(eval).sum::<usize>();
        result.push(print(slice.iter().map(eval)));
    }
    println!("Sum of difficulty {}", sum);
    result
}

fn edges<'a>(
    cave: &'a Vec<Vec<u8>>,
    state: &'a State,
    mut buffer: Vec<(State, usize)>,
) -> Vec<(State, usize)> {
    let works = |tool: Tool, pos: (usize, usize)| match cave[pos.1][pos.0] {
        b'.' => tool != Tool::Neither,
        b'=' => tool != Tool::Torch,
        b'|' => tool != Tool::Climbing,
        _ => panic!("Arghs"),
    };
    buffer.clear();
    for &tool in [Tool::Neither, Tool::Torch, Tool::Climbing].iter() {
        if tool != state.tool && works(tool, state.pos) {
            buffer.push((State::new(tool, state.pos), 7));
        }
    }
    let neighbour = |x, y, buffer: &mut Vec<(State, usize)>| {
        if works(state.tool, (x, y)) {
            buffer.push((State::new(state.tool, (x, y)), 1));
        }
    };
    if state.pos.0 > 0 {
        neighbour(state.pos.0 - 1, state.pos.1, &mut buffer);
    }
    if state.pos.0 + 1 < cave[state.pos.1].len() {
        neighbour(state.pos.0 + 1, state.pos.1, &mut buffer);
    }
    if state.pos.1 > 0 {
        neighbour(state.pos.0, state.pos.1 - 1, &mut buffer);
    }
    if state.pos.1 + 1 < cave.len() {
        neighbour(state.pos.0, state.pos.1 + 1, &mut buffer);
    }
    buffer
}

fn path(cave: &Vec<Vec<u8>>, target: (usize, usize)) -> Option<usize> {
    let mut pq = PriorityQueue::new();
    let mut seen = HashMap::new();
    let mut buffer = Vec::new();
    let start = State {
        tool: Tool::Torch,
        pos: (0, 0),
    };
    seen.insert(start, 0);
    pq.push(start, RevOrd(0));
    while let Some((state, RevOrd(cost))) = pq.pop() {
        if State::new(Tool::Torch, target) == state {
            return Some(cost);
        }
        buffer = edges(&cave, &state, buffer);
        for (target, edge_cost) in &buffer {
            let target_cost = cost + edge_cost;
            match seen.entry(*target) {
                hash_map::Entry::Vacant(x) => {
                    x.insert(target_cost);
                    pq.push(*target, RevOrd(target_cost));
                }
                hash_map::Entry::Occupied(mut x) => {
                    if *x.get() > target_cost {
                        *x.get_mut() = target_cost;
                        pq.change_priority(target, RevOrd(target_cost));
                    }
                }
            }
        }
    }
    None
}

fn main() {
    println!(
        "{}",
        to_strings(generate((10, 10), (10, 10), 510)).join("\n")
    );
    println!(
        "Path: {:?}",
        path(&generate((10, 10), (20, 20), 510), (10, 10))
    );
    generate((7, 782), (7, 782), 11820);
    println!(
        "Path: {:?}",
        path(&generate((7, 782), (1000, 1000), 11820), (7, 782))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let expected: Vec<_> = "M=.|=.|.|=.|=|=.
.|=|=|||..|.=...
.==|....||=..|==
=.|....|.==.|==.
=|..==...=.|==..
=||.=.=||=|=..|=
|.=.===|||..=..|
|..==||=.|==|===
.=..===..=|.|||.
.======|||=|=.|=
.===|=|===T===||
=|||...|==..|=.|
=.=|=.=..=.||==|
||=|=...|==.=|==
|=.=||===.|||===
||.|==.|.|.||=||"
            .split("\n")
            .collect();
        let mut result = generate((10, 10), (15, 15), 510);
        result[0][0] = b'M';
        result[10][10] = b'T';
        assert_eq!(expected, to_strings(result));
    }
}

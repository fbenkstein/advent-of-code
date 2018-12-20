use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, prelude::*};

fn insert(
    result: &mut Vec<((isize, isize), (isize, isize))>,
    positions: &mut Vec<HashSet<(isize, isize)>>,
    i: usize,
    dx: isize,
    dy: isize,
) {
    let (first, other) = positions.split_at_mut(i + 1);
    for pos in &first[i] {
        let next = (pos.0 + dx, pos.1 + dy);
        other[0].insert(next);
        result.push((*pos, next));
    }
}

fn find_ors(input: impl Iterator<Item = (usize, u8)>) -> impl Iterator<Item = usize> {
    input
        .scan(0_isize, |count, (pos, c)| {
            if c == b'(' || c == b'^' {
                *count += 1;
            }
            if c == b')' || c == b'$' {
                *count -= 1;
            }
            if *count == 0 {
                return None;
            }
            if c == b'|' && count.abs() == 1 {
                Some(Some(pos))
            } else {
                Some(None)
            }
        })
        .filter_map(|x| x)
}

fn trace(input: &[u8]) -> Vec<((isize, isize), (isize, isize))> {
    let mut result = Vec::new();
    let mut positions = vec![HashSet::new(); input.len()];
    positions[0].insert((0, 0));
    for (i, &c) in input.iter().enumerate() {
        match c {
            b'N' => insert(&mut result, &mut positions, i, 0, -1),
            b'S' => insert(&mut result, &mut positions, i, 0, 1),
            b'W' => insert(&mut result, &mut positions, i, -1, 0),
            b'E' => insert(&mut result, &mut positions, i, 1, 0),
            b'(' | b'^' => {
                positions[i + 1] = positions[i].clone();
                for or in find_ors(input.iter().cloned().enumerate().skip(i)) {
                    positions[or + 1] = positions[i].clone()
                }
            }
            b')' => {
                positions[i + 1] = positions[i].clone();
                for or in find_ors(input[..=i].iter().cloned().enumerate().rev()) {
                    let copy = positions[or].clone();
                    positions[i + 1].extend(copy);
                }
            }
            b'|' => (),
            b'$' => (),
            _ => panic!("Invalid input"),
        }
    }
    result
}

fn bfs(input: &Vec<((isize, isize), (isize, isize))>) -> (usize, usize) {
    let edges: HashMap<_, Vec<_>> = input
        .iter()
        .sorted()
        .iter()
        .group_by(|(from, _)| from)
        .into_iter()
        .map(|(k, v)| (k, v.map(|(_, to)| to).collect()))
        .collect();
    let mut visited = HashSet::new();
    visited.insert((0, 0));
    let mut q = VecDeque::new();
    q.push_back((0, (0, 0)));
    let mut max_dist = 0;
    let mut num_long = 0;
    while let Some((dist, next)) = q.pop_front() {
        if let Some(edges) = edges.get(&next) {
            for &&to in edges {
                if visited.insert(to) {
                    q.push_back((dist + 1, to));
                    max_dist = dist + 1;
                    if dist + 1 >= 1000 {
                        num_long += 1;
                    }
                }
            }
        }
    }
    (max_dist, num_long)
}

fn print(input: &Vec<((isize, isize), (isize, isize))>) -> Vec<Vec<u8>> {
    let (min, max) = input
        .iter()
        .map(|(from, _)| from)
        .chain(input.iter().map(|(_, to)| to))
        .minmax()
        .into_option()
        .unwrap();
    let dim = ((max.0 - min.0) as usize, (max.1 - min.1) as usize);
    let mut out = vec![vec![b'#'; dim.0 * 2 + 3]; dim.1 * 2 + 3];
    let coord = |x: (isize, isize)| ((x.0 - min.0) as usize, (x.1 - min.1) as usize);
    for &(from, to) in input {
        let (from, to) = (coord(from), coord(to));
        out[from.1 * 2 + 1][from.0 * 2 + 1] = b'.';
        out[to.1 * 2 + 1][to.0 * 2 + 1] = b'.';
        if from.1 != to.1 {
            out[from.1 + to.1 + 1][to.0 * 2 + 1] = b'-';
        } else {
            out[to.1 * 2 + 1][from.0 + to.0 + 1] = b'|';
        }
    }
    let origin = coord((0, 0));
    out[origin.1 * 2 + 1][origin.0 * 2 + 1] = b'X';
    out
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    for x in lines {
        let edges = trace(&x.as_bytes());
        println!("\n{}", x);
        for line in print(&edges) {
            println!("{}", std::str::from_utf8(&line).unwrap());
        }
        println!("Max distance: {:?}", bfs(&edges));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expected = "#####
#.|.#
#-###
#.|X#
#####";
        let edges = trace(b"^WNE$");
        let out = print(&edges);
        let out = out
            .iter()
            .map(|x| String::from_utf8(x.clone()).unwrap())
            .join("\n");
        assert_eq!(expected, out);
        assert_eq!(3, bfs(&edges).0);
    }

    #[test]
    fn test2() {
        let expected = "#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########";
        let edges = trace(b"^ENWWW(NEEE|SSE(EE|N))$");
        let out = print(&edges);
        let out = out
            .iter()
            .map(|x| String::from_utf8(x.clone()).unwrap())
            .join("\n");
        assert_eq!(expected, out);
        assert_eq!(10, bfs(&edges).0);
    }

    #[test]
    fn test3() {
        let expected = "###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########";
        let edges = trace(b"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        let out = print(&edges);
        let out = out
            .iter()
            .map(|x| String::from_utf8(x.clone()).unwrap())
            .join("\n");
        assert_eq!(expected, out);
        assert_eq!(18, bfs(&edges).0);
    }

    #[test]
    fn test4() {
        let expected = "#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############";
        let edges = trace(b"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        let out = print(&edges);
        let out = out
            .iter()
            .map(|x| String::from_utf8(x.clone()).unwrap())
            .join("\n");
        assert_eq!(expected, out);
        assert_eq!(23, bfs(&edges).0);
    }

    #[test]
    fn test5() {
        let expected = "###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############";
        let edges = trace(b"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        let out = print(&edges);
        let out = out
            .iter()
            .map(|x| String::from_utf8(x.clone()).unwrap())
            .join("\n");
        assert_eq!(expected, out);
        assert_eq!(31, bfs(&edges).0);
    }
}

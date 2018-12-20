use itertools::Itertools;

use std::collections::{HashSet, VecDeque};
use std::io::{self, prelude::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct P {
    x: isize,
    y: isize,
}

fn p(x: isize, y: isize) -> P {
    P { x, y }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Edge {
    from: P,
    to: P,
}

fn insert(result: &mut Vec<Edge>, positions: &mut Vec<P>, delta: P) {
    for pos in positions.iter_mut() {
        let from = *pos;
        pos.x += delta.x;
        pos.y += delta.y;
        result.push(Edge { from, to: *pos });
    }
}

fn trace(input: &[u8]) -> Vec<Edge> {
    let mut result = Vec::new();
    let mut stack = vec![(vec![p(0, 0)], Vec::new()); 1];
    for &c in input.iter() {
        match c {
            b'N' => insert(&mut result, &mut stack.last_mut().unwrap().0, p(0, -1)),
            b'S' => insert(&mut result, &mut stack.last_mut().unwrap().0, p(0, 1)),
            b'W' => insert(&mut result, &mut stack.last_mut().unwrap().0, p(-1, 0)),
            b'E' => insert(&mut result, &mut stack.last_mut().unwrap().0, p(1, 0)),
            b'(' | b'^' => stack.push((stack.last().unwrap().0.clone(), Vec::new())),
            b')' | b'$' => {
                let (positions, mut choices) = stack.pop().unwrap();
                choices.extend(positions);
                choices.sort();
                choices.dedup();
                stack.last_mut().unwrap().0 = choices;
            }
            b'|' => {
                let (positions, mut choices) = stack.pop().unwrap();
                choices.extend(positions);
                stack.push((stack.last().unwrap().0.clone(), choices));
            }
            _ => panic!("Invalid input"),
        }
    }
    result
}

fn get_edges<'a>(edges: &'a Vec<Edge>, from: P) -> impl Iterator<Item = P> + 'a {
    let to = p(isize::min_value(), isize::min_value());
    let edge = Edge { from, to };
    let start = edges.binary_search(&edge).unwrap_or_else(|x| x);
    let suffix = edges[start..].iter();
    suffix.take_while(move |e| e.from == from).map(|e| e.to)
}

fn bfs(input: &Vec<Edge>) -> (usize, usize) {
    let edges = input.iter().cloned().sorted();
    let mut visited: HashSet<_> = [p(0, 0)].iter().cloned().collect();
    let mut q: VecDeque<_> = [(0, p(0, 0))].iter().cloned().collect();
    let (mut max_dist, mut num_long) = (0, 0);
    while let Some((dist, next)) = q.pop_front() {
        for to in get_edges(&edges, next) {
            if visited.insert(to) {
                max_dist = dist + 1;
                q.push_back((max_dist, to));
                if max_dist >= 1000 {
                    num_long += 1;
                }
            }
        }
    }
    (max_dist, num_long)
}

fn print(input: &Vec<Edge>) -> Vec<Vec<u8>> {
    let froms = input.iter().map(|e| e.from);
    let tos = input.iter().map(|e| e.to);
    let (min, max) = froms.chain(tos).minmax().into_option().unwrap();
    let dim = ((max.x - min.x) as usize, (max.y - min.y) as usize);
    let mut out = vec![vec![b'#'; dim.0 * 2 + 3]; dim.1 * 2 + 3];
    let coord = |p: P| ((p.x - min.x) as usize, (p.y - min.y) as usize);
    for e in input {
        let (from, to) = (coord(e.from), coord(e.to));
        out[from.1 * 2 + 1][from.0 * 2 + 1] = b'.';
        out[to.1 * 2 + 1][to.0 * 2 + 1] = b'.';
        if from.1 != to.1 {
            out[from.1 + to.1 + 1][to.0 * 2 + 1] = b'-';
        } else {
            out[to.1 * 2 + 1][from.0 + to.0 + 1] = b'|';
        }
    }
    let origin = coord(p(0, 0));
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

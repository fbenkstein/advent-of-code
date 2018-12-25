use itertools::Itertools;
use std::collections::HashMap;

use std::io::{self, prelude::*};

fn parse(input: &Vec<String>) -> Vec<(isize, isize, isize, isize)> {
    input
        .iter()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().unwrap())
                .tuple_windows::<(_, _, _, _)>()
                .next()
                .unwrap()
        })
        .collect()
}

fn neighbours(
    pos: (isize, isize, isize, isize),
) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    (-3_isize..=3)
        .flat_map(|x| (-3 + x.abs()..=3 - x.abs()).map(move |y| (x, y)))
        .flat_map(|(x, y)| (-3 + x.abs() + y.abs()..=3 - x.abs() - y.abs()).map(move |z| (x, y, z)))
        .flat_map(|(x, y, z)| {
            (-3 + x.abs() + y.abs() + z.abs()..=3 - x.abs() - y.abs() - z.abs())
                .map(move |t| (x, y, z, t))
        })
        .map(move |(x, y, z, t)| (pos.0 + x, pos.1 + y, pos.2 + z, pos.3 + t))
}

#[derive(Debug)]
struct UnionFind {
    data: Vec<isize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            data: vec![-1; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut result = x;
        while self.data[result] >= 0 {
            result = self.data[result] as usize;
        }

        let mut path = x;
        while self.data[path] >= 0 {
            let next = self.data[path] as usize;
            self.data[path] = result as isize;
            path = next;
        }
        result
    }

    fn union(&mut self, left: usize, right: usize) -> bool {
        let left_root = self.find(left);
        let right_root = self.find(right);
        if left_root != right_root {
            if self.data[left_root] < self.data[right_root] {
                self.data[right_root] = left_root as isize;
            } else if self.data[left_root] > self.data[right_root] {
                self.data[left_root] = right_root as isize;
            } else {
                self.data[left_root] = right_root as isize;
                self.data[right_root] -= 1;
            }
            true
        } else {
            false
        }
    }
}

fn solve(input: &Vec<(isize, isize, isize, isize)>) -> usize {
    let lookup: HashMap<_, _> = input
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
    let mut unions = UnionFind::new(input.len());
    for (x_pos, x) in input.iter().enumerate() {
        for y in neighbours(*x) {
            if let Some(y_pos) = lookup.get(&y) {
                unions.union(x_pos, *y_pos);
            }
        }
    }
    (0..input.len())
        .filter(|pos| unions.find(*pos) == *pos)
        .count()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    let num_unions = solve(&input);
    println!("Num constellations: {}", num_unions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0"
            .split("\n")
            .map(|x| x.into())
            .collect();
        let parsed = parse(&input);
        let num_unions = solve(&parsed);
        assert_eq!(num_unions, 2);
    }

    #[test]
    fn test2() {
        let input = "-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0"
            .split("\n")
            .map(|x| x.into())
            .collect();
        let parsed = parse(&input);
        let num_unions = solve(&parsed);
        assert_eq!(num_unions, 4);
    }

    #[test]
    fn test3() {
        let input = "1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2"
            .split("\n")
            .map(|x| x.into())
            .collect();
        let parsed = parse(&input);
        let num_unions = solve(&parsed);
        assert_eq!(num_unions, 3);
    }

    #[test]
    fn test4() {
        let input = "1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2"
            .split("\n")
            .map(|x| x.into())
            .collect();
        let parsed = parse(&input);
        let num_unions = solve(&parsed);
        assert_eq!(num_unions, 8);
    }
}

use itertools::Itertools;
use text_io::{scan, try_scan};

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn parse(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let from: char;
            let to: char;
            scan!(line.bytes() => "Step {} must be finished before step {} can begin.", from, to);
            (from, to)
        })
        .sorted()
}

// Entry for BinaryHeap to inverse sorting -> min heap
#[derive(Debug, PartialEq, Eq)]
struct MinOrd<T: PartialEq + Eq + PartialOrd + Ord>(T);

impl<T: PartialEq + Eq + PartialOrd + Ord> Ord for MinOrd<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl<T: PartialEq + Eq + PartialOrd + Ord> PartialOrd for MinOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Lexicographic topological sort
fn lex_top_sort(edges: &[(char, char)]) -> String {
    let mut out_edges = HashMap::new();
    let mut in_degrees = HashMap::new();
    for &(from, to) in edges {
        let out = out_edges.entry(from).or_insert_with(Vec::new);
        out.push(to);
        in_degrees.entry(from).or_insert(0);
        let deg = in_degrees.entry(to).or_insert(0);
        *deg += 1;
    }

    let mut zero_degrees: BinaryHeap<MinOrd<_>> = in_degrees
        .iter()
        .filter_map(|(&v, &deg)| if deg == 0 { Some(MinOrd(v)) } else { None })
        .collect();

    let mut res = String::new();
    while let Some(MinOrd(v)) = zero_degrees.pop() {
        if let Some(out) = out_edges.get(&v) {
            for w in out {
                let deg = in_degrees.get_mut(w).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    zero_degrees.push(MinOrd(*w));
                }
            }
        }
        res.push(v);
    }
    res
}

pub fn simulation(edges: &[(char, char)], num_workers: usize, task_duration: usize) -> usize {
    let mut out_edges = HashMap::new();
    let mut in_degrees = HashMap::new();
    for &(from, to) in edges {
        let out = out_edges.entry(from).or_insert_with(Vec::new);
        out.push(to);
        in_degrees.entry(from).or_insert(0);
        let deg = in_degrees.entry(to).or_insert(0);
        *deg += 1;
    }

    let mut zero_degrees: BinaryHeap<MinOrd<_>> = in_degrees
        .iter()
        .filter_map(|(&v, &deg)| if deg == 0 { Some(MinOrd(v)) } else { None })
        .collect();
    let mut events = BinaryHeap::new();

    let mut t = 0; // current time
    let mut num_occupied_workers = 0;
    while !zero_degrees.is_empty() || !events.is_empty() {
        // add new event for each ready to execute task as long we have enough workers
        while !zero_degrees.is_empty() && num_occupied_workers < num_workers {
            let MinOrd(task) = zero_degrees.pop().unwrap();
            events.push(MinOrd((
                t + task_duration + (task as u8 - 'A' as u8 + 1) as usize,
                task,
            )));
            num_occupied_workers += 1;
        }

        if let Some(MinOrd((t_task, task))) = events.pop() {
            t = t_task;
            num_occupied_workers -= 1;

            if let Some(out) = out_edges.get(&task) {
                for w in out {
                    let deg = in_degrees.get_mut(w).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        zero_degrees.push(MinOrd(*w));
                    }
                }
            }
        }
    }

    t
}

pub fn solve(input: &str) -> (String, usize) {
    let edges = parse(input);
    (lex_top_sort(&edges), simulation(&edges, 5, 60))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT),
            vec![
                ('A', 'B'),
                ('A', 'D'),
                ('B', 'E'),
                ('C', 'A'),
                ('C', 'F'),
                ('D', 'E'),
                ('F', 'E')
            ]
        );
    }

    #[test]
    fn test_order() {
        assert_eq!(lex_top_sort(&vec![('A', 'C'), ('B', 'C')]), "ABC");
        assert_eq!(
            lex_top_sort(&vec![('A', 'B'), ('B', 'C'), ('A', 'C')]),
            "ABC"
        );
        assert_eq!(
            lex_top_sort(&vec![('A', 'B'), ('A', 'C'), ('A', 'D'), ('C', 'E')]),
            "ABCDE"
        );
        assert_eq!(lex_top_sort(&parse(INPUT)), "CABDFE");
    }

    #[test]
    fn test_simulation() {
        assert_eq!(simulation(&parse(INPUT), 2, 0), 15);
    }
}

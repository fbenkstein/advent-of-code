#[macro_use]
extern crate text_io;
extern crate itertools;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Node(char);

fn main() {
    let mut edges = include_str!("input_07.txt")
        .lines()
        .map(|s| {
            let x: char;
            let y: char;
            scan!(s.bytes() => "Step {} must be finished before step {} can begin.", x, y);
            (Node(x), Node(y))
        })
        .fold(HashMap::new(), |mut edges, (from_node, to_node)| {
            edges
                .entry(from_node)
                .or_insert_with(HashSet::new)
                .insert(to_node);
            edges.entry(to_node).or_default();
            edges
        });

    let mut output = String::new();

    while !edges.is_empty() {
        let roots: Vec<_> = edges
            .keys()
            .filter(|from_node| {
                edges
                    .values()
                    .map(|to_nodes| !to_nodes.contains(from_node))
                    .all(|b| b)
            })
            .cloned()
            .sorted();
        assert!(!roots.is_empty(), "cycle detected");
        edges.retain(|to_node, _| !roots.contains(to_node));
        output.extend(roots.into_iter().map(|Node(c)| c));
    }

    println!("{}", output);
}

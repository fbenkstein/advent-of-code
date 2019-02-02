#[macro_use]
extern crate text_io;
extern crate itertools;

use std::collections::{HashMap, HashSet};

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
        let smallest_root = edges
            .keys()
            .filter(|from_node| {
                edges
                    .values()
                    .map(|to_nodes| !to_nodes.contains(from_node))
                    .all(|b| b)
            })
            .min()
            .cloned()
            .expect("no roots! cycles?");
        edges.remove(&smallest_root);
        output.push(smallest_root.0);
    }

    println!("{}", output);
}

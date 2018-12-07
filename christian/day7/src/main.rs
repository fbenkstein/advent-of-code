#[macro_use]
extern crate text_io;
use revord::RevOrd;
use std::collections::BinaryHeap;
use std::io::{self, prelude::*};

#[derive(Default, Debug, Ord, PartialOrd, PartialEq, Eq)]
struct Edge {
    from: usize,
    to: usize,
}

fn parse(input: &Vec<String>) -> Vec<Edge> {
    let convert = |line: &String| {
        let (from, to): (char, char);
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.",from,to);
        Edge {
            from: (from as u8 - b'A') as usize,
            to: (to as u8 - b'A') as usize,
        }
    };
    input.iter().map(convert).collect()
}

fn get_edges(edges: &Vec<Edge>, from: usize) -> impl Iterator<Item = &Edge> {
    let edge = &Edge { from, to: 0 };
    let start = edges.binary_search(edge).unwrap_or_else(|x| x);
    let suffix = edges[start..].iter();
    suffix.take_while(move |edge| edge.from == from)
}

fn create_in_edge_count(edges: &Vec<Edge>) -> Vec<usize> {
    let max = edges.iter().map(|x| x.to).max().unwrap() + 1;
    let max = max.max(edges.iter().map(|x| x.from).max().unwrap() + 1);
    let mut num_in_edges = vec![0; max];
    for edge in edges {
        num_in_edges[edge.to] += 1;
    }
    num_in_edges
}

// This is a topological sort on steroids:
// It supports "delay" for each node
// It supports only having a certain amount of work "in progress" in parallel
// The simple version has one thing in progress, and no delay
fn solve(input: &Vec<Edge>, worker: usize, delay: impl Fn(usize, usize) -> usize) {
    let mut num_in_edges = create_in_edge_count(input);

    let start_iter = num_in_edges.iter().enumerate().filter(|(_, x)| **x == 0);
    let mut queue: BinaryHeap<_> = start_iter.map(|(x, _)| RevOrd(x)).collect();

    let mut in_progress = BinaryHeap::new();
    let add_work = |time, in_progress: &mut BinaryHeap<_>, queue: &mut BinaryHeap<_>| {
        for RevOrd(x) in (in_progress.len()..worker).flat_map(|_| queue.pop()) {
            in_progress.push(RevOrd((delay(time, x), x)));
        }
    };

    add_work(0, &mut in_progress, &mut queue);
    let mut last_time = 0;
    while let Some(RevOrd((time, node))) = in_progress.pop() {
        last_time = time;
        print!("{}", (b'A' + node as u8) as char);
        for edge in get_edges(input, node) {
            num_in_edges[edge.to] -= 1;
            if num_in_edges[edge.to] == 0 {
                queue.push(RevOrd(edge.to));
            }
        }
        add_work(time, &mut in_progress, &mut queue);
    }
    println!("");
    println!("Took {} time steps", last_time);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut input = parse(&lines);
    input.sort();
    println!("One worker, no delay");
    solve(&input, 1, |_, _| 0);
    println!("Two worker, 1+x delay");
    solve(&input, 2, |time, node| time + 1 + node);
    println!("Five worker, 61+x delay");
    solve(&input, 5, |time, node| time + 61 + node);
}

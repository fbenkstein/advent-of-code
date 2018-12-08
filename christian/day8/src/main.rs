#[macro_use]
extern crate text_io;
use std::io::{self, prelude::*};

#[derive(Default, Debug)]
struct Node {
    meta: Vec<usize>,
    children: Vec<Box<Node>>,
}

fn parse(mut input: &mut impl Iterator<Item = u8>) -> Box<Node> {
    let mut result: Node = Default::default();
    let (num_children, num_meta): (usize, usize);
    scan!(input => "{} {}",num_children,num_meta);
    result.children = (0..num_children).map(|_| parse(input)).collect();
    result.meta = (0..num_meta).map(|_| read!("{}", input)).collect();
    Box::new(result)
}

fn sum(node: &Box<Node>) -> usize {
    node.children.iter().map(sum).sum::<usize>() + node.meta.iter().sum::<usize>()
}

fn solve2(node: &Box<Node>) -> usize {
    if node.children.len() == 0 {
        return node.meta.iter().sum::<usize>();
    }
    node.meta
        .iter()
        .filter_map(|x| node.children.get(x.wrapping_sub(1)))
        .map(solve2)
        .sum::<usize>()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let tree = parse(&mut lines.first().unwrap().bytes());
    println!("Sum: {:?}", sum(&tree));
    println!("Part2: {:?}", solve2(&tree));
}

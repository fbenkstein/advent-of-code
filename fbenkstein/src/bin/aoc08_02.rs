#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn parse<T: Iterator<Item = u32>>(data: &mut T) -> Node {
        let child_count = data.next().expect("child count");
        let metadata_count = data.next().expect("metadata count");
        let children = (0..child_count).map(|_| Node::parse(data)).collect();
        let metadata = (0..metadata_count).filter_map(|_| data.next()).collect();
        Node { children, metadata }
    }

    fn sum(&self) -> u32 {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|x| if *x > 0 { Some(x - 1) } else { None })
                .filter_map(|x| self.children.get(x as usize))
                .map(|child| child.sum())
                .sum()
        }
    }
}

fn main() {
    let input = include_str!("input_08.txt");
    let mut data = input.trim().split(" ").map(|s| s.parse().unwrap());
    let root: Node = Node::parse(&mut data);

    println!("{}", root.sum())
}

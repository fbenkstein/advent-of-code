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

    fn flatten(self) -> Vec<Node> {
        let mut nodes = Vec::new();
        nodes.push(self);
        let mut i = 0;

        while i < nodes.len() {
            let more_nodes = nodes[i].children.clone();
            nodes.extend(more_nodes);
            i += 1;
        }

        nodes
    }
}

fn main() {
    let input = include_str!("input_08.txt");
    let mut data = input.trim().split(" ").map(|s| s.parse().unwrap());
    let root: Node = Node::parse(&mut data);

    println!(
        "{}",
        root.flatten()
            .iter()
            .flat_map(|c| c.metadata.iter())
            .sum::<u32>()
    )
}

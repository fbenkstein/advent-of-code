#[derive(Debug)]
pub struct License {
    node: Node,
}

impl License {
    pub fn decode(input: &str) -> Self {
        let data: Vec<u8> = input.split(" ").map(|c| c.parse().unwrap()).collect();
        Self {
            node: Self::read_node(&mut data.into_iter()),
        }
    }

    fn read_node(data: &mut Iterator<Item = u8>) -> Node {
        let qty_nodes = data.next().expect("Could not decode nodes length");
        let qty_metadata = data.next().expect("Could not decode metadata length");
        Node {
            nodes: (0..qty_nodes).map(|_| Self::read_node(data)).collect(),
            metadata: (0..qty_metadata)
                .map(|_| data.next().expect("Could not decode metadata entry"))
                .collect(),
        }
    }

    pub fn part_one(&self) -> usize {
        self.node.part1_sum(0)
    }

    pub fn part_two(&self) -> usize {
        self.node.value(0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    metadata: Vec<u8>,
    nodes: Vec<Node>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        self.metadata.iter().map(|i| *i as usize).sum::<usize>()
    }

    fn part1_sum(&self, sum: usize) -> usize {
        let mut total = 0;
        for node in &self.nodes {
            total += node.part1_sum(sum);
        }
        // TODO: find a better way to have type inference with sum?
        sum + total + self.metadata_sum()
    }

    fn value(&self, sum: usize) -> usize {
        let mut total = 0;
        if self.nodes.len() == 0 {
            sum + self.metadata_sum()
        } else {
            sum + self
                .metadata
                .iter()
                .map(|m| {
                    if let Some(node) = self.nodes.get(*m as usize - 1) {
                        node.value(sum)
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        }
    }
}

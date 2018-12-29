use nom::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
enum Pattern {
    Directions(Vec<char>),
    Group(Vec<Pattern>),
    Seq(Vec<Pattern>),
    Empty,
}

named!(directions<&[u8], Pattern>,
    map!(
        many1!(alt!(char!('N') | char!('S') | char!('W') | char!('E'))),
        Pattern::Directions
    )
);

named!(group<&[u8], Pattern>,
    do_parse!(
        char!('(') >>
        first: pattern >>
        rest: many1!(preceded!(char!('|'), opt!(pattern))) >>
        char!(')') >>
        ({
            let mut res = vec![first];
            res.extend(rest.into_iter().map(|p| p.unwrap_or(Pattern::Empty)));
            Pattern::Group(res)
        })
    )
);

named!(pattern<&[u8], Pattern>,
    map!(many1!(alt!(group | directions)), Pattern::Seq)
);

named!(parse<&[u8], Pattern>,
    delimited!(char!('^'), pattern, char!('$'))
);

impl Pattern {
    fn walk(&self, graph: &mut Graph, origins: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
        match self {
            Pattern::Seq(patterns) => patterns
                .iter()
                .fold(origins, |origins, p| p.walk(graph, origins)),
            Pattern::Directions(dirs) => dirs.iter().fold(origins, |mut origins, dir| {
                for (i, j) in origins.iter_mut() {
                    match dir {
                        'E' => {
                            let neighbors = graph.entry((*i, *j)).or_insert_with(Vec::new);
                            neighbors.push((*i, *j + 1));
                            *j += 1;
                        }
                        'W' => {
                            let neighbors = graph.entry((*i, *j)).or_insert_with(Vec::new);
                            neighbors.push((*i, *j - 1));
                            *j -= 1;
                        }
                        'N' => {
                            let neighbors = graph.entry((*i, *j)).or_insert_with(Vec::new);
                            neighbors.push((*i - 1, *j));
                            *i -= 1;
                        }
                        'S' => {
                            let neighbors = graph.entry((*i, *j)).or_insert_with(Vec::new);
                            neighbors.push((*i + 1, *j));
                            *i += 1;
                        }
                        _ => panic!("unknown direction"),
                    }
                }
                origins
            }),
            Pattern::Group(subpatterns) => {
                let mut origins = subpatterns.iter().fold(Vec::new(), |mut new_origins, p| {
                    new_origins.extend(p.walk(graph, origins.clone()).iter());
                    new_origins
                });
                origins.sort_unstable();
                origins.dedup();
                origins
            }
            Pattern::Empty => origins,
        }
    }
}

type Graph = HashMap<(isize, isize), Vec<(isize, isize)>>;

fn shortest_paths(graph: &Graph) -> HashMap<(isize, isize), usize> {
    type Vertex = (isize, isize);
    let mut stack: Vec<Vertex> = vec![(0, 0)];
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut dist: HashMap<Vertex, usize> = HashMap::new();
    dist.insert((0, 0), 0);

    // DFS
    while let Some(v) = stack.pop() {
        if let Some(neighbors) = graph.get(&v) {
            for w in neighbors {
                let dist_vw = dist[&v] + 1;
                let dist_w = dist.entry(*w).or_insert(usize::max_value());
                if dist_vw < *dist_w {
                    *dist_w = dist_vw
                }
                if !visited.contains(&w) {
                    stack.push(*w);
                }
            }
        }
        visited.insert(v);
    }
    dist
}

pub fn solve(input: &str) -> (usize, usize) {
    let (_, pattern) = parse(input.as_bytes()).expect("parser failed");
    let mut graph = Graph::new();
    pattern.walk(&mut graph, vec![(0, 0)]);
    let dist = shortest_paths(&graph);
    let max_shortest_path = dist.iter().map(|(_, &d)| d).max().unwrap();
    let num_rooms_shortest_path_with_at_least_1000_doors = dist
        .iter()
        .filter_map(|(_, &d)| if d >= 1000 { Some(d) } else { None })
        .count();
    (
        max_shortest_path,
        num_rooms_shortest_path_with_at_least_1000_doors,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_shortest_path1() {
        const INPUT: &str = "^WNE$";
        let (_, pattern) = parse(INPUT.as_bytes()).expect("parser failed");
        let mut graph = Graph::new();
        pattern.walk(&mut graph, vec![(0, 0)]);
        let dist = shortest_paths(&graph);
        let max_shortest_path = dist.iter().map(|(_, &d)| d).max().unwrap();
        assert_eq!(max_shortest_path, 3);
    }

    #[test]
    fn test_max_shortest_path2() {
        const INPUT: &str = "^ENWWW(NEEE|SSE(EE|N))$";
        let (_, pattern) = parse(INPUT.as_bytes()).expect("parser failed");
        let mut graph = Graph::new();
        pattern.walk(&mut graph, vec![(0, 0)]);
        let dist = shortest_paths(&graph);
        let max_shortest_path = dist.iter().map(|(_, &d)| d).max().unwrap();
        assert_eq!(max_shortest_path, 10);
    }

    #[test]
    fn test_max_shortest_path3() {
        const INPUT: &str = "^N(E|W)S$";
        let (_, pattern) = parse(INPUT.as_bytes()).expect("parser failed");
        let mut graph = Graph::new();
        pattern.walk(&mut graph, vec![(0, 0)]);
        let dist = shortest_paths(&graph);
        let max_shortest_path = dist.iter().map(|(_, &d)| d).max().unwrap();
        assert_eq!(max_shortest_path, 3);
    }

    #[test]
    fn test_max_shortest_path4() {
        const INPUT: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let (_, pattern) = parse(INPUT.as_bytes()).expect("parser failed");
        let mut graph = Graph::new();
        pattern.walk(&mut graph, vec![(0, 0)]);
        let dist = shortest_paths(&graph);
        let max_shortest_path = dist.iter().map(|(_, &d)| d).max().unwrap();
        assert_eq!(max_shortest_path, 18);
    }
}

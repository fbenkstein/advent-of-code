use priority_queue::PriorityQueue;
use revord::RevOrd;
use std::collections::{btree_map, BTreeMap};
use std::usize;
use text_io::{scan, try_scan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Cave {
    target: Coord,
    depth: usize,
    erosion_levels: BTreeMap<Coord, usize>,
}

impl Cave {
    fn new(target: Coord, depth: usize) -> Self {
        Self {
            target,
            depth,
            erosion_levels: BTreeMap::new(),
        }
    }

    fn calc_geological_index(&mut self, pos: Coord) -> usize {
        if pos.x == 0 && pos.y == 0 {
            0
        } else if pos == self.target {
            0
        } else if pos.y == 0 {
            pos.x * 16807
        } else if pos.x == 0 {
            pos.y * 48271
        } else {
            self.erosion_level(Coord::new(pos.x - 1, pos.y))
                * self.erosion_level(Coord::new(pos.x, pos.y - 1))
        }
    }

    fn erosion_level(&mut self, pos: Coord) -> usize {
        if !self.erosion_levels.contains_key(&pos) {
            let level = (self.calc_geological_index(pos) + self.depth) % 20183;
            self.erosion_levels.insert(pos, level);
            level
        } else {
            self.erosion_levels[&pos]
        }
    }

    fn erosion_type(&mut self, pos: Coord) -> ErosionType {
        ErosionType::from_level(self.erosion_level(pos) % 3)
    }

    fn risk_level(&mut self) -> usize {
        let mut risk = 0;
        for x in 0..=self.target.x {
            for y in 0..=self.target.y {
                risk += ErosionType::from_level(self.erosion_level(Coord::new(x, y))) as usize;
            }
        }
        risk
    }

    fn shortest_path_to_target(&mut self) -> usize {
        let mut dist = BTreeMap::new();
        let mut pq = PriorityQueue::new();

        let root = (Tool::Torch, Coord::new(0, 0));
        dist.insert(root, 0);
        pq.push(root, RevOrd(0));

        while let Some((v, v_cost)) = pq.pop() {
            if v.0 == Tool::Torch && v.1 == self.target {
                return v_cost.0;
            }

            if dist[&v] < v_cost.0 {
                continue; // better cost known
            }

            for (w, vw_cost) in self.edges(v) {
                let w_cost = v_cost.0 + vw_cost;
                match dist.entry(w) {
                    btree_map::Entry::Vacant(e) => {
                        pq.push(w, RevOrd(w_cost));
                        e.insert(w_cost);
                    }
                    btree_map::Entry::Occupied(ref mut e) if w_cost < *e.get() => {
                        pq.change_priority(&w, RevOrd(w_cost));
                        e.insert(w_cost);
                    }
                    _ => (),
                }
            }
        }
        usize::MAX // unreachable
    }

    // All edges from tool + pos with their corresponding costs.
    fn edges(&mut self, (tool, pos): (Tool, Coord)) -> Vec<((Tool, Coord), usize)> {
        let x = pos.x as isize;
        let y = pos.y as isize;

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        let neighbors = neighbors.into_iter().filter_map(|&(x, y)| {
            if 0 <= x && 0 <= y {
                Some(Coord::new(x as usize, y as usize))
            } else {
                None
            }
        });

        let mut edges: Vec<_> = neighbors
            .filter_map(|next_pos| match self.erosion_type(next_pos) {
                ErosionType::Rocky if tool == Tool::Torch || tool == Tool::ClimbingGear => {
                    Some(((tool, next_pos), 1))
                }
                ErosionType::Wet if tool == Tool::Neither || tool == Tool::ClimbingGear => {
                    Some(((tool, next_pos), 1))
                }
                ErosionType::Narrow if tool == Tool::Neither || tool == Tool::Torch => {
                    Some(((tool, next_pos), 1))
                }
                _ => None,
            })
            .collect();

        match self.erosion_type(pos) {
            ErosionType::Rocky => {
                for tool in [Tool::Torch, Tool::ClimbingGear]
                    .into_iter()
                    .filter(|t| **t != tool)
                {
                    edges.push(((*tool, pos), 7));
                }
            }
            ErosionType::Wet => {
                if tool == Tool::Neither {
                    edges.push(((Tool::ClimbingGear, pos), 7));
                } else {
                    edges.push(((Tool::Neither, pos), 7));
                }
            }
            ErosionType::Narrow => {
                if tool == Tool::Neither {
                    edges.push(((Tool::Torch, pos), 7));
                } else {
                    edges.push(((Tool::Neither, pos), 7));
                }
            }
        };

        edges
    }
}

enum ErosionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl ErosionType {
    fn from_level(level: usize) -> Self {
        match level % 3 {
            0 => ErosionType::Rocky,
            1 => ErosionType::Wet,
            2 => ErosionType::Narrow,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let depth;
    let target_x;
    let target_y;
    scan!(lines.next().unwrap().bytes() => "depth: {}", depth);
    scan!(lines.next().unwrap().bytes() => "target: {},{}", target_x, target_y);
    let mut cave = Cave::new(Coord::new(target_x, target_y), depth);

    let risk = cave.risk_level();
    let cost = cave.shortest_path_to_target();
    (risk, cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erosition_level() {
        let mut cave = Cave::new(Coord::new(10, 10), 510);
        assert_eq!(cave.erosion_level(Coord::new(0, 0)), 510);
        assert_eq!(cave.erosion_level(Coord::new(1, 0)), 17317);
        assert_eq!(cave.erosion_level(Coord::new(0, 1)), 8415);
        assert_eq!(cave.erosion_level(Coord::new(1, 1)), 1805);
        assert_eq!(cave.erosion_level(Coord::new(10, 10)), 510);
    }

    #[test]
    fn test_risk() {
        let mut cave = Cave::new(Coord::new(10, 10), 510);
        assert_eq!(cave.risk_level(), 114);
    }

    #[test]
    fn test_shortest_path_to_target() {
        let mut cave = Cave::new(Coord::new(10, 10), 510);
        let cost = cave.shortest_path_to_target();
        assert_eq!(cost, 45);
    }
}

use priority_queue::PriorityQueue;
use revord::RevOrd;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet, VecDeque};
use std::fmt;

struct Cave {
    depth: usize,
    width: usize,
    height: usize,
    target_x: usize,
    target_y: usize,
    regions: Vec<usize>,
}

enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl From<usize> for Region {
    fn from(erosion_level: usize) -> Region {
        match erosion_level % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither,
}

// impl From<Region> for Tool {
//     fn from(region: Region) -> Tool {
//         match region {

//         }
//     }
// }

impl Cave {
    fn new(depth: usize, (target_x, target_y): (usize, usize)) -> Cave {
        let width = target_x + 6;
        let height = target_y + 6;
        let mut regions: Vec<usize> = vec![0; width * height];
        for y in 0..height {
            for x in 0..width {
                let geological_index = if y == 0 && x == 0 {
                    0
                } else if y == target_y && x == target_y {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    regions[(x - 1) + y * width] * regions[x + (y - 1) * width]
                };
                regions[x + y * width] = (geological_index + depth) % 20183;
            }
        }
        Cave {
            depth,
            height,
            width,
            target_x,
            target_y,
            regions: regions,
        }
    }

    fn risk_level(&self) -> usize {
        let mut risk = 0;
        for x in 0..=self.target_x {
            for y in 0..=self.target_y {
                risk += self[(x, y)] % 3;
            }
        }
        risk
    }

    fn pathfind(&self) -> usize {
        // find all reachable tiles for this unit
        let mut reachable = BTreeMap::new();
        let mut queue = PriorityQueue::new();
        let start = ((0, 0), Tool::Torch);
        reachable.insert(start, 0);
        queue.push(start, RevOrd(0));

        while let Some((((x, y), tool), cost)) = queue.pop() {
            // special stop conditions
            if tool == Tool::Torch && (x, y) == (self.target_x, self.target_y) {
                return cost.0;
            }

            if reachable[&((x, y), tool)] < cost.0 {
                continue;
            }

            for (edge, edge_cost) in self.edges(((x, y), tool)) {
                let new_cost = cost.0 + edge_cost;
                reachable
                    .entry(edge)
                    .and_modify(|e| {
                        queue.change_priority(&edge, RevOrd(new_cost));
                        *e = new_cost
                    })
                    .or_insert_with(|| {
                        queue.push(edge, RevOrd(new_cost));
                        new_cost
                    });
            }
        }

        0
    }

    /// return edges with position, tool to switch to and cost
    fn edges(
        &self,
        ((x, y), tool): ((usize, usize), Tool),
    ) -> Vec<(((usize, usize), Tool), usize)> {
        let x = x as isize;
        let y = y as isize;
        let mut edges: Vec<(((usize, usize), Tool), usize)> =
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter_map(|&(x, y)| {
                    if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
                        let x = x as usize;
                        let y = y as usize;
                        match Region::from(self[(x, y)]) {
                            Region::Rocky if tool == Tool::Torch || tool == Tool::ClimbingGear => {
                                Some((((x, y), tool), 1))
                            }
                            Region::Wet if tool == Tool::ClimbingGear || tool == Tool::Neither => {
                                Some((((x, y), tool), 1))
                            }
                            Region::Narrow if tool == Tool::Torch || tool == Tool::Neither => {
                                Some((((x, y), tool), 1))
                            }
                            _ => None, // need to switch tool
                        }
                    } else {
                        None
                    }
                })
                .collect();

        // check whether we need to switch tool
        let x = x as usize;
        let y = y as usize;
        match Region::from(self[(x, y)]) {
            Region::Rocky => {
                if tool != Tool::Torch {
                    edges.push((((x, y), Tool::Torch), 7));
                }
                if tool != Tool::ClimbingGear {
                    edges.push((((x, y), Tool::ClimbingGear), 7));
                }
            }
            Region::Wet => {
                if tool != Tool::ClimbingGear {
                    edges.push((((x, y), Tool::ClimbingGear), 7));
                }
                if tool != Tool::Neither {
                    edges.push((((x, y), Tool::Neither), 7));
                }
            }
            Region::Narrow => {
                if tool != Tool::Torch {
                    edges.push((((x, y), Tool::Torch), 7));
                }
                if tool != Tool::ClimbingGear {
                    edges.push((((x, y), Tool::ClimbingGear), 7));
                }
            }
        }

        if (x, y) == (self.target_x, self.target_y) {
            edges.push((((x, y), Tool::Torch), 7));
        }

        edges
    }
}

use std::ops::Index;

impl Index<(usize, usize)> for Cave {
    type Output = usize;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.regions[x + y * (self.width)]
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if x == 0 && y == 0 {
                        'M'
                    } else if x == self.target_x && y == self.target_y {
                        'T'
                    } else {
                        match self[(x, y)] % 3 {
                            0 => '.',
                            1 => '=',
                            2 => '|',
                            _ => unreachable!(),
                        }
                    }
                )?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        writeln!(f, "Risk level: {}", self.risk_level())?;
        Ok(())
    }
}

fn main() {
    // let cave = Cave::new(510, (10, 10));
    // println!("{}", cave);

    let cave = Cave::new(4845, (6, 770));
    println!("{}", cave);

    println!("{}", cave.pathfind());
}

#[test]
fn example() {
    let cave = Cave::new(510, (10, 10));
    assert_eq!(cave[(0, 0)], 510);
    assert_eq!(cave[(1, 0)], 17317);
    assert_eq!(cave[(0, 1)], 8415);
    assert_eq!(cave[(1, 1)], 1805);
    assert_eq!(cave[(10, 10)], 510);

    assert_eq!(cave.risk_level(), 114);
}

#[test]
fn pathfinding() {
    let cave = Cave::new(510, (10, 10));
    assert_eq!(cave.pathfind(), 45);
}

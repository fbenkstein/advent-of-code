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

impl Region {
    fn from(depth: usize, geological_index: usize) -> Region {
        match (geological_index + depth) % 20183 % 3 {
            0 => Region::Rocky,
            1 => Region::Wet,
            2 => Region::Narrow,
            _ => unreachable!(),
        }
    }
}

impl Cave {
    fn new(depth: usize, (target_x, target_y): (usize, usize)) -> Cave {
        let width = target_x + 1;
        let height = target_y + 1;
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
    let cave = Cave::new(4845, (6, 770));
    println!("{}", cave);
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

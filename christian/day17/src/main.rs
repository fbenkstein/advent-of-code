#[macro_use]
extern crate text_io;
#[macro_use]
extern crate itertools;

use termion::{clear, cursor, screen::AlternateScreen};

use std::io::{self, prelude::*, stdout, Write};
use std::ops::Range;

fn parse(lines: &Vec<String>) -> Area {
    let parse_line = |line: &String| {
        let try_x = || -> Result<(Range<usize>, Range<usize>), text_io::Error> {
            let (x, y_start, y_end): (usize, usize, usize);
            try_scan!(line.bytes() => "x={}, y={}..{}", x, y_start, y_end);
            Ok((x..x + 1, y_start..y_end + 1))
        };
        let try_y = || -> Result<(Range<usize>, Range<usize>), text_io::Error> {
            let (x_start, x_end, y): (usize, usize, usize);
            try_scan!(line.bytes() => "y={}, x={}..{}", y, x_start, x_end);
            Ok((x_start..x_end + 1, y..y + 1))
        };
        try_x().or(try_y()).unwrap()
    };
    let mut input: Vec<_> = lines.iter().map(parse_line).collect();
    let min_x = input.iter().map(|(x, _)| x.start - 1).min().unwrap();
    for range in &mut input {
        range.0.start -= min_x;
        range.0.end -= min_x;
    }
    let width = input.iter().map(|(x, _)| x.end + 3).max().unwrap();
    let height = input.iter().map(|(_, y)| y.end).max().unwrap();
    let mut result = Area {
        width,
        height,
        data: vec![Block::Empty; width * height],
    };
    for (x_range, y_range) in input {
        for pos in iproduct!(x_range, y_range) {
            *result.block_mut(pos) = Block::Stone;
        }
    }
    *result.block_mut((500 - min_x, 0)) = Block::Spring;
    result
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    Flow,
    Water,
    Stone,
    Spring,
}

#[derive(Debug, Clone, Default)]
struct Area {
    data: Vec<Block>,
    width: usize,
    height: usize,
}

impl Area {
    fn block(&self, (x, y): (usize, usize)) -> Block {
        self.data[x + y * self.width]
    }

    fn block_mut(&mut self, (x, y): (usize, usize)) -> &mut Block {
        &mut self.data[x + y * self.width]
    }

    fn print(&self, screen: &mut impl Write, change_pos: usize, min_y: usize) -> usize {
        let max_height = self
            .height
            .min(termion::terminal_size().unwrap().1 as usize + 1);
        let mut min_y = self.height.saturating_sub(max_height).min(min_y);
        if change_pos < min_y || change_pos >= min_y + max_height {
            min_y = self
                .height
                .saturating_sub(max_height)
                .min(change_pos.saturating_sub(10));
        }
        write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        for y in min_y..min_y + max_height {
            for x in 0..self.width {
                let out = match self.block((x, y)) {
                    Block::Stone => '#',
                    Block::Empty => ' ',
                    Block::Spring => '+',
                    Block::Water => '~',
                    Block::Flow => '|',
                };
                write!(screen, "{}", out).unwrap();
            }
            writeln!(screen, "").unwrap();
        }
        screen.flush().unwrap();
        min_y
    }

    fn is_blocked(&self, (x, y): (usize, usize)) -> bool {
        match self.block((x, y)) {
            Block::Water | Block::Stone => true,
            _ => false,
        }
    }

    fn find<Positions>(&self, iter: Positions, y: usize) -> (Block, usize)
    where
        Positions: Iterator<Item = usize>,
    {
        let mut last_x = None;
        for x in iter {
            if self.block((x, y)) == Block::Stone {
                return (Block::Stone, last_x.unwrap());
            }
            last_x = Some(x);
            if !self.is_blocked((x, y + 1)) {
                return (Block::Flow, x);
            }
        }
        panic!("Did not find anything");
    }

    fn step(&mut self) -> usize {
        let mut min_y_changed = self.height;
        for y in 1..self.height - 1 {
            for x in 0..self.width {
                if self.block((x, y)) != Block::Flow {
                    continue;
                }
                if !self.is_blocked((x, y + 1)) {
                    if self.block((x, y + 1)) != Block::Flow {
                        min_y_changed = min_y_changed.min(y + 1);
                    }
                    *self.block_mut((x, y + 1)) = Block::Flow;
                    continue;
                }
                let (left, right) = (self.find((0..=x).rev(), y), self.find(x..self.width, y));
                let block = if left.0 == Block::Stone && right.0 == Block::Stone {
                    min_y_changed = min_y_changed.min(y + 1);
                    Block::Water
                } else {
                    Block::Flow
                };
                for x in left.1..=right.1 {
                    *self.block_mut((x, y)) = block;
                }
            }
        }
        for x in 0..self.width {
            if self.block((x, 0)) == Block::Spring {
                *self.block_mut((x, 1)) = Block::Flow;
            }
        }
        min_y_changed
    }

    fn reachable(&self) -> usize {
        let min_stone = self.data.iter().position(|x| *x == Block::Stone).unwrap();
        let valid_blocks = self.data[min_stone / self.width * self.width..].iter();
        valid_blocks
            .filter(|&&x| x == Block::Water || x == Block::Flow)
            .count()
    }

    fn num_water(&self) -> usize {
        self.data.iter().filter(|x| **x == Block::Water).count()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let mut area = parse(&lines);

    {
        let mut screen = AlternateScreen::from(stdout());
        let mut print_pos: usize = 0;
        let mut change_pos: usize = 0;
        for _ in 0..1000 {
            print_pos = area.print(&mut screen, change_pos, print_pos);
            change_pos = area.step();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    area.print(&mut stdout(), area.height, area.height);
    println!("Reachable: {}", area.reachable());
    println!("Water: {}", area.num_water());
}

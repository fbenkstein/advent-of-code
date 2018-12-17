use std::cmp::{Ord, Ordering};
use std::fmt::{self, Write};
use std::iter;
use std::mem;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cart {
    pos: (usize, usize),
    dir: Direction,
    state: usize,
}

// Inverse ordering by pos since we will pop carts from a stack
impl Ord for Cart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos).reverse()
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    fn next_pos(&self) -> (usize, usize) {
        match self.dir {
            Direction::Up => (self.pos.0 - 1, self.pos.1),
            Direction::Down => (self.pos.0 + 1, self.pos.1),
            Direction::Left => (self.pos.0, self.pos.1 - 1),
            Direction::Right => (self.pos.0, self.pos.1 + 1),
        }
    }
}

impl Into<char> for &Cart {
    fn into(self) -> char {
        match self.dir {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug)]
struct Configuration {
    tracks: Vec<char>,
    width: usize,
    height: usize,
    carts: Vec<Cart>,
}

impl Configuration {
    fn first_crash(mut self: Configuration) -> (usize, usize) {
        loop {
            if let Some(crash) = self.tick().first() {
                break (crash.1, crash.0);
            }
        }
    }

    fn last_car(mut self: Configuration) -> (usize, usize) {
        loop {
            self.tick();
            if self.carts.len() == 1 {
                let pos = self.carts[0].pos;
                break (pos.1, pos.0);
            }
        }
    }

    fn tick(self: &mut Configuration) -> Vec<(usize, usize)> {
        let mut stack = Vec::new();
        mem::swap(&mut stack, &mut self.carts);
        stack.sort_unstable(); // carts are sorted inverted s.t. we can pop them in order

        let mut crash_positions = Vec::new();

        while let Some(mut cart) = stack.pop() {
            let next_pos = cart.next_pos();
            let track = self.tracks[next_pos.0 * self.width + next_pos.1];
            match (track, &cart.dir) {
                ('\\', Direction::Up) => cart.dir = Direction::Left,
                ('\\', Direction::Down) => cart.dir = Direction::Right,
                ('\\', Direction::Left) => cart.dir = Direction::Up,
                ('\\', Direction::Right) => cart.dir = Direction::Down,
                ('/', Direction::Up) => cart.dir = Direction::Right,
                ('/', Direction::Down) => cart.dir = Direction::Left,
                ('/', Direction::Left) => cart.dir = Direction::Down,
                ('/', Direction::Right) => cart.dir = Direction::Up,
                ('+', dir) => {
                    match cart.state {
                        0 => cart.dir = dir.turn_left(),
                        2 => cart.dir = dir.turn_right(),
                        _ => (),
                    }
                    cart.state = (cart.state + 1) % 3;
                }
                _ => (),
            }
            cart.pos = next_pos;

            if let Some(idx) = self // test for a crash in prev cars
                .carts
                .iter()
                .position(|other_cart| cart.pos == other_cart.pos)
            {
                crash_positions.push(cart.pos);
                self.carts.swap_remove(idx);
            } else if let Some(idx) = stack // test for a crash in next cars
                .iter()
                .position(|other_cart| cart.pos == other_cart.pos)
            {
                crash_positions.push(cart.pos);
                stack.remove(idx); // remove preserving order
            } else {
                self.carts.push(cart);
            }
        }
        crash_positions
    }
}

fn parse(input: &str) -> Configuration {
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let height = input.lines().count();

    let mut tracks = Vec::new();
    let mut carts = Vec::new();
    for (i, line) in input.lines().enumerate() {
        tracks.extend(line.chars().enumerate().map(|(j, c)| match c {
            '^' | 'v' => {
                carts.push(Cart {
                    pos: (i, j),
                    dir: Direction::from(c),
                    state: 0,
                });
                '|'
            }
            '<' | '>' => {
                carts.push(Cart {
                    pos: (i, j),
                    dir: Direction::from(c),
                    state: 0,
                });
                '-'
            }
            other => other,
        }));
        tracks.extend(iter::repeat(' ').take(width - line.len()));
    }

    Configuration {
        tracks,
        width,
        height,
        carts,
    }
}

impl fmt::Display for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                if let Some(cart) = self.carts.iter().find(|cart| cart.pos == (i, j)) {
                    f.write_char(cart.into())?
                } else {
                    let track = self.tracks[i * self.width + j];
                    f.write_char(track)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(input: &str) -> ((usize, usize), (usize, usize)) {
    let conf = parse(input);
    let conf2 = parse(input);
    (conf.first_crash(), conf2.last_car())
}

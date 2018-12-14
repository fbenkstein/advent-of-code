use std::io::{self, prelude::*};

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum Field {
    Empty,
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
    Intersection,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
enum State {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
struct Cart {
    x: usize,
    y: usize,
    id: usize,
    state: State,
    dir: Direction,
}

#[derive(Debug, Clone)]
struct Input {
    tracks: Vec<Vec<Field>>,
    carts: Vec<Cart>,
}

fn parse(input: &Vec<String>) -> Input {
    let mut carts = Vec::new();
    let mut id = 0;
    let convert = |c| match c {
        b'>' => Some(Direction::Right),
        b'<' => Some(Direction::Left),
        b'^' => Some(Direction::Up),
        b'v' => Some(Direction::Down),
        _ => None,
    };
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if let Some(dir) = convert(c) {
                carts.push(Cart {
                    x,
                    y,
                    id,
                    state: State::Left,
                    dir,
                });
                id += 1;
            }
        }
    }
    let convert = |c| match c {
        b' ' => Field::Empty,
        b'-' | b'>' | b'<' => Field::Horizontal,
        b'|' | b'^' | b'v' => Field::Vertical,
        b'/' => Field::DiagonalUp,
        b'\\' => Field::DiagonalDown,
        b'+' => Field::Intersection,
        _ => panic!("Input kaputt"),
    };
    let tracks: Vec<Vec<Field>> = input
        .iter()
        .map(|line| line.bytes().map(convert).collect())
        .collect();

    Input { tracks, carts }
}

impl Cart {
    fn step(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
    fn choose(&mut self) {
        self.state = match self.state {
            State::Left => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                State::Straight
            }
            State::Straight => State::Right,
            State::Right => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                State::Left
            }
        };
    }

    fn diagonal_up(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
    }

    fn diagonal_down(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }
}

fn solve(input: &Input, first_crash: bool) {
    let mut carts: Vec<_> = input.carts.iter().cloned().collect();
    carts.sort();
    let mut occupied: Vec<Vec<Option<usize>>> =
        vec![vec![None; input.tracks[0].len()]; input.tracks.len()];
    for cart in &carts {
        occupied[cart.y][cart.x] = Some(cart.id);
    }
    while carts.len() > 1 {
        for cart in &mut carts {
            match occupied[cart.y][cart.x] {
                Some(other) if other == cart.id => {}
                _ => continue,
            }
            occupied[cart.y][cart.x] = None;
            cart.step();
            if let Some(_) = occupied[cart.y][cart.x] {
                if first_crash {
                    println!("First crash @ ({},{})", cart.x, cart.y);
                    return;
                }
                occupied[cart.y][cart.x] = None;
                continue;
            }
            occupied[cart.y][cart.x] = Some(cart.id);

            match &input.tracks[cart.y][cart.x] {
                Field::DiagonalUp => cart.diagonal_up(),
                Field::DiagonalDown => cart.diagonal_down(),
                Field::Intersection => cart.choose(),
                _ => {}
            }
        }
        carts.retain(|cart| match occupied[cart.y][cart.x] {
            Some(other) if other == cart.id => true,
            _ => false,
        });
        carts.sort();
    }
    if let Some(survivor) = carts.first() {
        println!("Last car alive @ ({},{})", survivor.x, survivor.y);
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    solve(&input, true);
    solve(&input, false);
}

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn print_tracks(tracks: &[char], carts: &[Cart], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if let Some(cart) = carts.iter().find(|c| c.x == x && c.y == y) {
                print!("{}", cart.state);
            } else {
                print!("{}", tracks[x + y * width]);
            }
        }
        print!("\n");
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cart {
    pub x: usize,
    pub y: usize,
    pub state: Direction,
    pub turns: usize,
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

impl From<char> for Direction {
    fn from(state: char) -> Self {
        match state {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("booh"),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c: char = (*self).into();
        write!(f, "{}", c)
    }
}

impl Cart {
    fn next(&mut self, tracks: &[char], width: usize) {
        match self.state {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }

        let current_rail = tracks[self.x + self.y * width];
        self.state = match current_rail {
            '-' | '|' => self.state,
            '/' => match self.state {
                Direction::Up => self.state.turn_right(),
                Direction::Left => self.state.turn_left(),
                Direction::Down => self.state.turn_right(),
                Direction::Right => self.state.turn_left(),
            },
            '\\' => match self.state {
                Direction::Up => self.state.turn_left(),
                Direction::Right => self.state.turn_right(),
                Direction::Left => self.state.turn_right(),
                Direction::Down => self.state.turn_left(),
            },
            '+' => {
                let new_state = match self.turns % 3 {
                    0 => self.state.turn_left(),
                    1 => self.state,
                    2 => self.state.turn_right(),
                    _ => panic!("wat"),
                };
                self.turns += 1;
                new_state
            }
            _ => panic!("derailed!!!!"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let height = contents.lines().count();
    let width = contents
        .lines()
        .map(|line| line.len())
        .max()
        .expect("no max?!");

    let mut tracks = Vec::with_capacity(height);
    let mut carts: Vec<Cart> = Vec::new();

    for (y, line) in contents.lines().enumerate() {
        let mut rails = Vec::with_capacity(width);
        for (x, rail) in line.chars().enumerate() {
            // we put the actual rail under the train
            match rail {
                '>' | '<' => {
                    carts.push(Cart {
                        x: x,
                        y: y,
                        state: Direction::from(rail),
                        turns: 0,
                    });
                    rails.push('-');
                }
                '^' | 'v' => {
                    carts.push(Cart {
                        x: x,
                        y: y,
                        state: Direction::from(rail),
                        turns: 0,
                    });
                    rails.push('|');
                }
                _ => rails.push(rail),
            }
        }
        for i in line.chars().count()..width {
            rails.insert(i, ' ');
        }
        tracks.extend(rails);
    }

    let mut visited_carts: HashMap<(usize, usize), usize> = HashMap::new();
    loop {
        visited_carts.clear();
        for (idx, cart) in carts.iter_mut().enumerate() {
            cart.next(&tracks, width);
            if visited_carts.insert((cart.x, cart.y), idx).is_some() {
                println!(
                    "Crash occurred at position {},{} - sending carts to maintenance.",
                    cart.x, cart.y
                );
                visited_carts.remove(&(cart.x, cart.y));
            }
        }
        carts.retain(|cart| visited_carts.contains_key(&(cart.x, cart.y)));
        if carts.len() == 1 {
            print_tracks(&tracks, &carts, width, height);
            println!("Only one cart remains! {:#?}", carts.first().unwrap());
            return Ok(());
        }
    }
}

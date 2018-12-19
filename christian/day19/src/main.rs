#[macro_use]
extern crate text_io;
use itertools::Itertools;
use std::str::FromStr;

use std::io::{self, prelude::*};

#[derive(Debug, Clone)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, Clone)]
struct Instruction {
    opcode: Opcode,
    d: [usize; 3],
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct State {
    r: [usize; 6],
    i: usize,
}

fn parse_instruction(line: &String) -> Instruction {
    let (opcode, d0, d1, d2): (String, usize, usize, usize);
    scan!(line.bytes() => "{} {} {} {}", opcode, d0, d1, d2);
    let opcode = match &opcode[..] {
        "addr" => Opcode::Addr,
        "addi" => Opcode::Addi,
        "mulr" => Opcode::Mulr,
        "muli" => Opcode::Muli,
        "banr" => Opcode::Banr,
        "bani" => Opcode::Bani,
        "borr" => Opcode::Borr,
        "bori" => Opcode::Bori,
        "setr" => Opcode::Setr,
        "seti" => Opcode::Seti,
        "gtir" => Opcode::Gtir,
        "gtri" => Opcode::Gtri,
        "gtrr" => Opcode::Gtrr,
        "eqir" => Opcode::Eqir,
        "eqri" => Opcode::Eqri,
        "eqrr" => Opcode::Eqrr,
        _ => panic!("Invalid opcode"),
    };
    Instruction {
        opcode,
        d: [d0, d1, d2],
    }
}

fn parse(input: &Vec<String>) -> (State, Vec<Instruction>) {
    let convert = |line: &String| parse_instruction(line);
    let ip: usize;
    let ip_line = &input[0];
    scan!(ip_line.bytes() => "#ip {}", ip);
    (
        State { r: [0; 6], i: ip },
        input[1..].iter().map(convert).collect(),
    )
}

fn exec(state: &mut State, i: &Instruction) {
    match i.opcode {
        Opcode::Addr => state.r[i.d[2]] = state.r[i.d[0]] + state.r[i.d[1]],
        Opcode::Addi => state.r[i.d[2]] = state.r[i.d[0]] + i.d[1],
        Opcode::Mulr => state.r[i.d[2]] = state.r[i.d[0]] * state.r[i.d[1]],
        Opcode::Muli => state.r[i.d[2]] = state.r[i.d[0]] * i.d[1],
        Opcode::Banr => state.r[i.d[2]] = state.r[i.d[0]] & state.r[i.d[1]],
        Opcode::Bani => state.r[i.d[2]] = state.r[i.d[0]] & i.d[1],
        Opcode::Borr => state.r[i.d[2]] = state.r[i.d[0]] | state.r[i.d[1]],
        Opcode::Bori => state.r[i.d[2]] = state.r[i.d[0]] | i.d[1],
        Opcode::Setr => state.r[i.d[2]] = state.r[i.d[0]],
        Opcode::Seti => state.r[i.d[2]] = i.d[0],
        Opcode::Gtir => state.r[i.d[2]] = (i.d[0] > state.r[i.d[1]]) as usize,
        Opcode::Gtri => state.r[i.d[2]] = (state.r[i.d[0]] > i.d[1]) as usize,
        Opcode::Gtrr => state.r[i.d[2]] = (state.r[i.d[0]] > state.r[i.d[1]]) as usize,
        Opcode::Eqir => state.r[i.d[2]] = (i.d[0] == state.r[i.d[1]]) as usize,
        Opcode::Eqri => state.r[i.d[2]] = (state.r[i.d[0]] == i.d[1]) as usize,
        Opcode::Eqrr => state.r[i.d[2]] = (state.r[i.d[0]] == state.r[i.d[1]]) as usize,
    }
}

fn print(indent: bool, pos: usize, state: &State, i: &Instruction) -> (bool, String) {
    let r = |x: usize| {
        if x == state.i {
            format!("Goto 1 +")
        } else {
            format!("r{} =", x)
        }
    };
    let v = |x: usize| {
        if x == state.i {
            format!("{}", pos)
        } else {
            format!("r{}", x)
        }
    };
    let binary = |r: String, left: String, op: char, right: String| {
        let (&left, &right) = [&left, &right].iter().minmax().into_option().unwrap();
        if r == "Goto 1 +" && left == &format!("{}", pos) && right.starts_with("r") && op == '+' {
            return format!("if !{}", right);
        }
        if r == "Goto 1 +" && !left.starts_with("r") && !right.starts_with("r") && op == '+' {
            return format!(
                "Goto {:02}",
                usize::from_str(left).unwrap() + usize::from_str(right).unwrap() + 1
            );
        }
        if r[0..2] != *left && r[0..2] != *right {
            return format!("{} {} {} {}", r, left, op, right);
        }
        let other = if r[0..2] != *left { left } else { right };
        format!("{} {}= {}", &r[0..2], op, other)
    };
    let set = |r: String, other: String| {
        if r == "Goto 1 +" && !other.starts_with("r") {
            return format!("Goto {:02}", usize::from_str(&other).unwrap() + 1);
        }

        format!("{} {}", r, other)
    };
    let result = match i.opcode {
        Opcode::Addr => binary(r(i.d[2]), v(i.d[0]), '+', v(i.d[1])),
        Opcode::Addi => binary(r(i.d[2]), v(i.d[0]), '+', format!("{}", i.d[1])),
        Opcode::Mulr => binary(r(i.d[2]), v(i.d[0]), '*', v(i.d[1])),
        Opcode::Muli => binary(r(i.d[2]), v(i.d[0]), '*', format!("{}", i.d[1])),
        Opcode::Banr => binary(r(i.d[2]), v(i.d[0]), '&', v(i.d[1])),
        Opcode::Bani => binary(r(i.d[2]), v(i.d[0]), '&', format!("{}", i.d[1])),
        Opcode::Borr => binary(r(i.d[2]), v(i.d[0]), '|', v(i.d[1])),
        Opcode::Bori => binary(r(i.d[2]), v(i.d[0]), '|', format!("{}", i.d[1])),
        Opcode::Setr => set(r(i.d[2]), v(i.d[0])),
        Opcode::Seti => set(r(i.d[2]), format!("{}", i.d[0])),
        Opcode::Gtir => format!("{} {} > {}", r(i.d[2]), i.d[0], v(i.d[1])),
        Opcode::Gtri => format!("{} {} > {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Gtrr => format!("{} {} > {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Eqir => format!("{} {} == {}", r(i.d[2]), i.d[0], v(i.d[1])),
        Opcode::Eqri => format!("{} {} == {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Eqrr => format!("{} {} == {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
    };
    let indent = if indent { "    " } else { " " };
    (
        result.starts_with("if"),
        format!("{:02}:{} {}", pos, indent, result),
    )
}

fn run((initial_state, program): &(State, Vec<Instruction>)) -> State {
    let mut state: State = initial_state.clone();
    let mut pos = 0;
    while pos < program.len() {
        state.r[state.i] = pos;
        exec(&mut state, &program[pos]);
        pos = state.r[state.i];
        pos += 1;
    }
    state
}

fn run_eq(num: usize) {
    let result: usize = (1..=num).filter(|&x| num % x == 0).sum();
    println!("Result {}", result);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    let res1 = run(&input);
    println!("Result {:?}", res1);
    let mut indent = false;
    let mut out = Vec::new();
    for (pos, x) in input.1.iter().enumerate() {
        let line = print(indent, pos, &input.0, x);
        indent = line.0;
        out.push(line.1);
    }
    let destinations: Vec<_> = out
        .iter()
        .filter_map(|line| {
            if let Some(pos) = line.find("Goto") {
                return Some(String::from_str(&line[pos + 5..]).unwrap());
            }
            None
        })
        .collect();
    for line in out.iter() {
        let prefix_pos = line.find(":").unwrap();
        if let None = destinations.iter().find(|x| **x == line[..prefix_pos]) {
            println!("    {}", &line[4..]);
        } else {
            println!("{}", line);
        }
    }
    run_eq(939);
    run_eq(10551339);
}

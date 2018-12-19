#[macro_use]
extern crate text_io;

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

fn print(pos: usize, state: &State, i: &Instruction) {
    let r = |x: usize| {
        if x == state.i {
            format!("{}: Goto 1+", pos)
        } else {
            format!("{}: r{} = ", pos, x)
        }
    };
    let v = |x: usize| {
        if x == state.i {
            format!("{}", pos)
        } else {
            format!("r{}", x)
        }
    };
    match i.opcode {
        Opcode::Addr => println!("{} {} + {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Addi => println!("{} {} + {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Mulr => println!("{} {} * {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Muli => println!("{} {} * {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Banr => println!("{} {} & {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Bani => println!("{} {} & {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Borr => println!("{} {} | {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Bori => println!("{} {} | {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Setr => println!("{} {}", r(i.d[2]), v(i.d[0])),
        Opcode::Seti => println!("{} {}", r(i.d[2]), i.d[0]),
        Opcode::Gtir => println!("{} {} > {}", r(i.d[2]), i.d[0], v(i.d[1])),
        Opcode::Gtri => println!("{} {} > {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Gtrr => println!("{} {} > {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
        Opcode::Eqir => println!("{} {} == {}", r(i.d[2]), i.d[0], v(i.d[1])),
        Opcode::Eqri => println!("{} {} == {}", r(i.d[2]), v(i.d[0]), i.d[1]),
        Opcode::Eqrr => println!("{} {} == {}", r(i.d[2]), v(i.d[0]), v(i.d[1])),
    }
}

fn run((initial_state, program): &(State, Vec<Instruction>)) {
    let mut state: State = initial_state.clone();
    let mut pos = 0;
    while pos < program.len() {
        state.r[state.i] = pos;
        exec(&mut state, &program[pos]);
        pos = state.r[state.i];
        pos += 1;
    }
    println!("Result {:?}", state);
}

fn run_eq(num: usize) {
    let result: usize = (1..=num).filter(|&x| num % x == 0).sum();
    println!("Result {}", result);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    for (pos, x) in input.1.iter().enumerate() {
        print(pos, &input.0, x);
    }
    run(&input);
    run_eq(939);
    run_eq(10551339);
}

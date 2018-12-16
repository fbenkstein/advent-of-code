#[macro_use]
extern crate text_io;
#[macro_use]
extern crate itertools;
use std::collections::HashSet;
use std::io::{self, prelude::*};

#[derive(Debug, Clone, Default)]
struct Test {
    before: [usize; 4],
    after: [usize; 4],
    instruction: [usize; 4],
}

#[derive(Debug, Clone, Default)]
struct Input {
    tests: Vec<Test>,
    program: Vec<[usize; 4]>,
}

fn parse_instruction(input: &String) -> Result<[usize; 4], text_io::Error> {
    let (i0, i1, i2, i3): (usize, usize, usize, usize);
    try_scan!(input.bytes() => "{} {} {} {}", i0,i1,i2,i3);
    Ok([i0, i1, i2, i3])
}

fn parse_state(input: &String) -> Result<[usize; 4], text_io::Error> {
    let (r0, r1, r2, r3): (usize, usize, usize, usize);
    let what: String;
    try_scan!(input.bytes().filter(|c| *c != b' ') => "{}:[{},{},{},{}]",what, r0,r1,r2,r3);
    assert!(what == "Before" || what == "After");
    Ok([r0, r1, r2, r3])
}

fn parse(input: &Vec<String>) -> Input {
    let mut result: Input = Default::default();
    let mut lines = input.iter().filter(|line| !line.is_empty());
    while let Some(line) = lines.next() {
        if let Ok(state) = parse_state(line) {
            result.tests.push(Test {
                before: state,
                instruction: parse_instruction(lines.next().unwrap()).unwrap(),
                after: parse_state(lines.next().unwrap()).unwrap(),
            });
        } else {
            result.program.push(parse_instruction(line).unwrap());
            break;
        }
    }
    for line in lines {
        result.program.push(parse_instruction(line).unwrap());
    }
    result
}

fn exec(opcode: usize, mut state: [usize; 4], inst: [usize; 4]) -> Option<[usize; 4]> {
    match opcode {
        0 => *state.get_mut(inst[3])? = *state.get(inst[1])? + *state.get(inst[2])?,
        1 => *state.get_mut(inst[3])? = *state.get(inst[1])? + inst[2],
        2 => *state.get_mut(inst[3])? = *state.get(inst[1])? * *state.get(inst[2])?,
        3 => *state.get_mut(inst[3])? = *state.get(inst[1])? * inst[2],
        4 => *state.get_mut(inst[3])? = *state.get(inst[1])? & *state.get(inst[2])?,
        5 => *state.get_mut(inst[3])? = *state.get(inst[1])? & inst[2],
        6 => *state.get_mut(inst[3])? = *state.get(inst[1])? | *state.get(inst[2])?,
        7 => *state.get_mut(inst[3])? = *state.get(inst[1])? | inst[2],
        8 => *state.get_mut(inst[3])? = *state.get(inst[1])?,
        9 => *state.get_mut(inst[3])? = inst[1],
        10 => *state.get_mut(inst[3])? = (inst[1] > *state.get(inst[2])?) as usize,
        11 => *state.get_mut(inst[3])? = (*state.get(inst[1])? > inst[2]) as usize,
        12 => *state.get_mut(inst[3])? = (*state.get(inst[1])? > *state.get(inst[2])?) as usize,
        13 => *state.get_mut(inst[3])? = (inst[1] == *state.get(inst[2])?) as usize,
        14 => *state.get_mut(inst[3])? = (*state.get(inst[1])? == inst[2]) as usize,
        15 => *state.get_mut(inst[3])? = (*state.get(inst[1])? == *state.get(inst[2])?) as usize,
        _ => return None,
    }
    Some(state)
}

fn get_mapping(input: &Input) -> [usize; 16] {
    let mut num_ambiguous = 0;
    let mut mapping: HashSet<_> = iproduct!(0..16, 0..16).collect();
    for test in &input.tests {
        let mut num_matching = 0;
        for opcode in 0..16 {
            if let Some(result) = exec(opcode, test.before, test.instruction) {
                if result[..] == test.after[..] {
                    num_matching += 1;
                } else {
                    mapping.remove(&(test.instruction[0], opcode));
                }
            } else {
                mapping.remove(&(test.instruction[0], opcode));
            }
        }
        if num_matching >= 3 {
            num_ambiguous += 1;
        }
    }

    println!(
        "Number of ambiguous: {} out of {}",
        num_ambiguous,
        input.tests.len()
    );

    // deduce rest
    let mut opcodes = [666; 16];
    loop {
        let mut something_happened = false;
        for opcode in 0..15 {
            let mut choices = mapping.iter().filter(|(from, _)| *from == opcode);
            if choices.clone().count() == 1 {
                // this mapping is fixed by only one from
                something_happened = true;
                let choice = choices.next().unwrap().1;
                mapping.retain(|(from, to)| *from != opcode && *to != choice);
                println!("Setting {} to {}", opcode, choice);
                opcodes[opcode] = choice;
            }
        }
        for opcode in 0..15 {
            let mut choices = mapping.iter().filter(|(_, to)| *to == opcode);
            if choices.clone().count() == 1 {
                // this mapping is fixed by only one to
                something_happened = true;
                let choice = choices.next().unwrap().0;
                mapping.retain(|(from, to)| *from != choice && *to != opcode);
                println!("Setting {} to {}", choice, opcode);
                opcodes[choice] = opcode;
            }
        }

        if !something_happened {
            break;
        }
    }

    for x in &opcodes {
        assert!(*x != 666);
    }
    opcodes
}

fn run(mapping: &[usize; 16], input: &Input) {
    let mut state = [0; 4];
    for x in &input.program {
        state = exec(mapping[x[0]], state, *x).unwrap();
    }
    println!("Result {:?}", state);
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|x| x.unwrap()).collect();
    let input = parse(&lines);
    let mapping = get_mapping(&input);
    run(&mapping, &input);
}

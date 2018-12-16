use std::fs::File;
use std::io::prelude::*;

use nom::types::CompleteStr;
use nom::*;

pub struct Test {
    before: [usize; 4],
    after: [usize; 4],
    instruction: Instruction,
}

struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[allow(non_camel_case_types)]
pub enum Opcode {
    addr(usize, usize, usize),
    addi(usize, usize, usize),
    mulr(usize, usize, usize),
    muli(usize, usize, usize),
    banr(usize, usize, usize),
    bani(usize, usize, usize),
    borr(usize, usize, usize),
    bori(usize, usize, usize),
    setr(usize, usize, usize),
    seti(usize, usize, usize),
    gtir(usize, usize, usize),
    gtri(usize, usize, usize),
    gtrr(usize, usize, usize),
    eqir(usize, usize, usize),
    eqri(usize, usize, usize),
    eqrr(usize, usize, usize),
}

impl Opcode {
    fn all(i: &Instruction) -> [Opcode; 16] {
        [
            Opcode::addr(i.a, i.b, i.c),
            Opcode::addi(i.a, i.b, i.c),
            Opcode::mulr(i.a, i.b, i.c),
            Opcode::muli(i.a, i.b, i.c),
            Opcode::banr(i.a, i.b, i.c),
            Opcode::bani(i.a, i.b, i.c),
            Opcode::borr(i.a, i.b, i.c),
            Opcode::bori(i.a, i.b, i.c),
            Opcode::setr(i.a, i.b, i.c),
            Opcode::seti(i.a, i.b, i.c),
            Opcode::gtir(i.a, i.b, i.c),
            Opcode::gtri(i.a, i.b, i.c),
            Opcode::gtrr(i.a, i.b, i.c),
            Opcode::eqir(i.a, i.b, i.c),
            Opcode::eqri(i.a, i.b, i.c),
            Opcode::eqrr(i.a, i.b, i.c),
        ]
    }

    fn from_instruction(i: &Instruction, vtable: &[usize; 16]) -> Self {
        let operation_pos = vtable[i.opcode];
        match operation_pos {
            0 => Opcode::addr(i.a, i.b, i.c),
            1 => Opcode::addi(i.a, i.b, i.c),
            2 => Opcode::mulr(i.a, i.b, i.c),
            3 => Opcode::muli(i.a, i.b, i.c),
            4 => Opcode::banr(i.a, i.b, i.c),
            5 => Opcode::bani(i.a, i.b, i.c),
            6 => Opcode::borr(i.a, i.b, i.c),
            7 => Opcode::bori(i.a, i.b, i.c),
            8 => Opcode::setr(i.a, i.b, i.c),
            9 => Opcode::seti(i.a, i.b, i.c),
            10 => Opcode::gtir(i.a, i.b, i.c),
            11 => Opcode::gtri(i.a, i.b, i.c),
            12 => Opcode::gtrr(i.a, i.b, i.c),
            13 => Opcode::eqir(i.a, i.b, i.c),
            14 => Opcode::eqri(i.a, i.b, i.c),
            15 => Opcode::eqrr(i.a, i.b, i.c),
            _ => panic!("INVALID OPCODE"),
        }
    }

    fn execute(&self, r: &mut [usize; 4]) {
        match *self {
            Opcode::addr(a, b, c) => r[c] = r[a] + r[b],
            Opcode::addi(a, b, c) => r[c] = r[a] + b,
            Opcode::mulr(a, b, c) => r[c] = r[a] * r[b],
            Opcode::muli(a, b, c) => r[c] = r[a] * b,
            Opcode::banr(a, b, c) => r[c] = r[a] & r[b],
            Opcode::bani(a, b, c) => r[c] = r[a] & b,
            Opcode::borr(a, b, c) => r[c] = r[a] | r[b],
            Opcode::bori(a, b, c) => r[c] = r[a] | b,
            Opcode::setr(a, _, c) => r[c] = r[a],
            Opcode::seti(a, _, c) => r[c] = a,
            Opcode::gtir(a, b, c) => r[c] = if a > r[b] { 1 } else { 0 },
            Opcode::gtri(a, b, c) => r[c] = if r[a] > b { 1 } else { 0 },
            Opcode::gtrr(a, b, c) => r[c] = if r[a] > r[b] { 1 } else { 0 },
            Opcode::eqir(a, b, c) => r[c] = if a == r[b] { 1 } else { 0 },
            Opcode::eqri(a, b, c) => r[c] = if r[a] == b { 1 } else { 0 },
            Opcode::eqrr(a, b, c) => r[c] = if r[a] == r[b] { 1 } else { 0 },
        }
    }
}

impl Test {
    fn count_opcodes_that_work(&self) -> usize {
        let after = self.after;
        Opcode::all(&self.instruction)
            .into_iter()
            .filter(|o| {
                let mut registers = self.before.clone();
                o.execute(&mut registers);
                registers == after
            })
            .count()
    }
}

named!(parse_usize(CompleteStr) -> usize,
    map_res!(
        do_parse!(
            number: digit >>
            opt!(tag!(",")) >>
            space0 >>
            (number)
        ), |input: CompleteStr| usize::from_str_radix(&input, 10)
    )
);

named!(pub parse_test<CompleteStr, Test>,
    do_parse!(
        opt!(line_ending) >>
        tag!("Before:") >>
        space0 >>
        tag!("[") >>
        before: count_fixed!(usize, parse_usize, 4) >>
        tag!("]") >>
        space0 >>
        line_ending >>
        opcode: parse_usize >>
        space0 >>
        a: parse_usize >>
        b: parse_usize >>
        c: parse_usize >>
        line_ending >>
        tag!("After:") >>
        space0 >>
        tag!("[") >>
        after: count_fixed!(usize, parse_usize, 4) >>
        tag!("]") >>
        opt!(line_ending) >>
        (Test { before, after, instruction: Instruction { opcode, a, b, c } })
    )
);

named!(pub parse_tests<CompleteStr, Vec<Test>>,
    do_parse!(
        tests: many0!(parse_test) >>
        (tests)
    )
);

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read file");
    let (_, tests) = parse_tests(CompleteStr(&contents)).expect("could not parse");

    println!(
        "Number of samples that behave like three or more opcodes: {}",
        tests.iter().filter(|&t| t.count_opcodes_that_work() >= 3).count()
    );

    // Part two
    // let mut opcodes: Vec<HashMap<String, usize>> = vec![];
    // for _ in 0..16 {
    //     opcodes.push(HashMap::new());
    // }
    // for test in tests.iter() {
    //     let compatible_opcodes = test.try_opcodes();
    //     for opcode in compatible_opcodes {
    //         *opcodes[test.opcode].entry(opcode).or_default() += 1;
    //     }
    // }

    // println!("{:?}", opcodes);
}

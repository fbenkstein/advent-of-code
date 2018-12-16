pub struct Test {
    before: [usize; 4],
    after: [usize; 4],
    opcode: usize,
    args: (usize, usize, usize),
}

impl Test {
    fn try_opcodes(&self) -> Vec<String> {
        let before = self.before;
        let after = self.after;
        let args = self.args;
        let mut possible_opcodes: Vec<String> = vec![];
        Self::try_opcode(&mut possible_opcodes, "addr", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] + r[b]
        });
        Self::try_opcode(&mut possible_opcodes, "addi", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] + b
        });
        Self::try_opcode(&mut possible_opcodes, "mulr", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] * r[b]
        });
        Self::try_opcode(&mut possible_opcodes, "muli", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] * b
        });
        Self::try_opcode(&mut possible_opcodes, "banr", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] & r[b]
        });
        Self::try_opcode(&mut possible_opcodes, "bani", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] & b
        });
        Self::try_opcode(&mut possible_opcodes, "borr", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] | r[b]
        });
        Self::try_opcode(&mut possible_opcodes, "bori", before, after, args, &|r, (a, b, c)| {
            r[c] = r[a] | b
        });
        Self::try_opcode(&mut possible_opcodes, "setr", before, after, args, &|r, (a, _, c)| {
            r[c] = r[a]
        });
        Self::try_opcode(&mut possible_opcodes, "seti", before, after, args, &|r, (a, _, c)| {
            r[c] = a
        });
        Self::try_opcode(&mut possible_opcodes, "gtir", before, after, args, &|r, (a, b, c)| {
            r[c] = if a > r[b] { 1 } else { 0 }
        });
        Self::try_opcode(&mut possible_opcodes, "gtri", before, after, args, &|r, (a, b, c)| {
            r[c] = if r[a] > b { 1 } else { 0 }
        });
        Self::try_opcode(&mut possible_opcodes, "gtrr", before, after, args, &|r, (a, b, c)| {
            r[c] = if r[a] > r[b] { 1 } else { 0 }
        });
        Self::try_opcode(&mut possible_opcodes, "eqir", before, after, args, &|r, (a, b, c)| {
            r[c] = if a == r[b] { 1 } else { 0 }
        });
        Self::try_opcode(&mut possible_opcodes, "eqri", before, after, args, &|r, (a, b, c)| {
            r[c] = if r[a] == b { 1 } else { 0 }
        });
        Self::try_opcode(&mut possible_opcodes, "eqrr", before, after, args, &|r, (a, b, c)| {
            r[c] = if r[a] == r[b] { 1 } else { 0 }
        });

        possible_opcodes
    }

    fn try_opcode(
        possible_opcodes: &mut Vec<String>,
        opcode: &str,
        before: [usize; 4],
        after: [usize; 4],
        args: (usize, usize, usize),
        operation: &Fn(&mut [usize; 4], (usize, usize, usize)),
    ) {
        let registers = &mut before.clone();
        operation(registers, args);
        if *registers == after {
            possible_opcodes.push(opcode.into());
        }
    }
}

use std::fs::File;
use std::io::prelude::*;

use nom::types::CompleteStr;
use nom::*;

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
        args: tuple!(parse_usize, parse_usize, parse_usize) >>
        line_ending >>
        tag!("After:") >>
        space0 >>
        tag!("[") >>
        after: count_fixed!(usize, parse_usize, 4) >>
        tag!("]") >>
        opt!(line_ending) >>
        (Test { before, after, opcode, args })
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
        tests.iter().filter(|t| t.try_opcodes().len() >= 3).count()
    );
}

pub use nom::types::CompleteStr;
use nom::*;

pub struct Test {
    pub before: [usize; 4],
    pub after: [usize; 4],
    pub instruction: Instruction,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    pub opcode: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
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

named!(pub parse_tests<CompleteStr, (Vec<Test>, Vec<Instruction>)>,
    do_parse!(
        tests: many0!(parse_test) >>
        many0!(line_ending) >>
        program_instructions: many0!(do_parse!(
            opcode: parse_usize >>
            a: parse_usize >>
            b: parse_usize >>
            c: parse_usize >>
            opt!(line_ending) >>
            (Instruction { opcode, a, b, c })
        )) >>
        ((tests, program_instructions))
    )
);

pub use nom::types::CompleteStr;
use nom::*;
use std::fmt;

pub type Registers = [usize; 6];

pub struct Instruction {
    opcode: String,
    a: usize,
    b: usize,
    c: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

impl Instruction {
    pub fn execute(&self, r: &mut Registers) {
        let (a, b, c) = (self.a, self.b, self.c);
        match self.opcode.as_ref() {
            "addr" => r[c] = r[a] + r[b],
            "addi" => r[c] = r[a] + b,
            "mulr" => r[c] = r[a] * r[b],
            "muli" => r[c] = r[a] * b,
            "banr" => r[c] = r[a] & r[b],
            "bani" => r[c] = r[a] & b,
            "borr" => r[c] = r[a] | r[b],
            "bori" => r[c] = r[a] | b,
            "setr" => r[c] = r[a],
            "seti" => r[c] = a,
            "gtir" => r[c] = if a > r[b] { 1 } else { 0 },
            "gtri" => r[c] = if r[a] > b { 1 } else { 0 },
            "gtrr" => r[c] = if r[a] > r[b] { 1 } else { 0 },
            "eqir" => r[c] = if a == r[b] { 1 } else { 0 },
            "eqri" => r[c] = if r[a] == b { 1 } else { 0 },
            "eqrr" => r[c] = if r[a] == r[b] { 1 } else { 0 },
            _ => panic!("Unsupported instruction!"),
        }
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

named!(pub parse_instruction<CompleteStr, Instruction>,
    do_parse!(
        opcode: map!(take!(4), |c: CompleteStr| c.to_string()) >>
        space0 >>
        a: parse_usize >>
        b: parse_usize >>
        c: parse_usize >>
        opt!(line_ending) >>
        (Instruction { opcode, a, b, c })
    )
);

named!(pub parse_instructions<CompleteStr, (usize, Vec<Instruction>)>,
    do_parse!(
        tag!("#ip") >>
        space0 >>
        instruction_pointer: parse_usize >>
        line_ending >>
        instructions: many0!(parse_instruction) >>
        ((instruction_pointer, instructions))
    )
);

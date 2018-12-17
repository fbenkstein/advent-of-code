use std::fs::File;
use std::io::prelude::*;

mod parser;
use crate::parser::*;

impl Opcode {
    fn all(i: Instruction) -> [Opcode; 16] {
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
        Opcode::all(self.instruction)
            .into_iter()
            .filter(|o| {
                let mut registers = self.before.clone();
                o.execute(&mut registers);
                registers == after
            })
            .count()
    }
}

fn resolve_opcodes(tests: &Vec<Test>) -> [usize; 16] {
    let mut vtable = [0 as usize; 16 * 16];
    for test in tests.iter() {
        Opcode::all(test.instruction)
            .into_iter()
            .enumerate()
            .filter(|(_, opcode)| {
                let mut registers = test.before.clone();
                opcode.execute(&mut registers);
                registers == test.after
            })
            .for_each(|(opcode_idx, _)| vtable[opcode_idx + test.instruction.opcode * 16] += 1);
    }

    let mut res: [usize; 16] = [0; 16];
    let mut unresolved_opcodes: Vec<usize> = (0..16).collect();
    while !unresolved_opcodes.is_empty() {
        // find opcode with unique max usages
        let (opcode_pos, &opcode, mapped_opcode) = unresolved_opcodes
            .iter()
            .enumerate()
            .filter_map(|(opcode_pos, opcode)| {
                let translation = &vtable[opcode * 16..(opcode + 1) * 16];
                // find max pos and value of this opcode
                let (max_pos, max_count) = translation.iter().enumerate().max_by_key(|(_, count)| *count).unwrap();
                // check if it is unique
                let is_unique = translation.iter().filter(|count| *count == max_count).count() == 1;
                if is_unique {
                    Some((opcode_pos, opcode, max_pos))
                } else {
                    None
                }
            })
            .next()
            .expect("some opcode should be resolvable");
        res[opcode] = mapped_opcode;
        // remove this mapping from all other opcodes i.e. from the column
        for opcode in 0..16 {
            vtable[opcode * 16 + mapped_opcode] = 0;
        }
        // removed this opcode from unresolved opcodes
        unresolved_opcodes.swap_remove(opcode_pos);
    }
    res
}

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read file");
    let (_, (tests, instructions)) = parse_tests(CompleteStr(&contents)).expect("could not parse");

    println!(
        "Number of samples that behave like three or more opcodes: {}",
        tests.iter().filter(|&t| t.count_opcodes_that_work() >= 3).count()
    );

    let vtable = resolve_opcodes(&tests);
    let mut registers = [0 as usize; 4];
    for instruction in instructions.iter() {
        Opcode::from_instruction(instruction, &vtable).execute(&mut registers);
    }

    println!(
        "Value is contained in register 0 after executing the test program: {}",
        registers[0]
    );
}

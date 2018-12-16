use text_io::{scan, try_scan};

type Registers = [i64; 4];

#[allow(non_camel_case_types)]
enum OpCode {
    addr(usize, usize, usize),
    addi(usize, i64, usize),
    mulr(usize, usize, usize),
    muli(usize, i64, usize),
    banr(usize, usize, usize),
    bani(usize, i64, usize),
    borr(usize, usize, usize),
    bori(usize, i64, usize),
    setr(usize, usize, usize),
    seti(i64, usize, usize),
    gtir(i64, usize, usize),
    gtri(usize, i64, usize),
    gtrr(usize, usize, usize),
    eqir(i64, usize, usize),
    eqri(usize, i64, usize),
    eqrr(usize, usize, usize),
}

impl OpCode {
    fn from_instr(instr: &Instruction, translation_table: &[usize; 16]) -> Self {
        let operation_pos = translation_table[instr.opcode];
        match operation_pos {
            0 => OpCode::addr(instr.a, instr.b, instr.c),
            1 => OpCode::addi(instr.a, instr.b as i64, instr.c),
            2 => OpCode::mulr(instr.a, instr.b, instr.c),
            3 => OpCode::muli(instr.a, instr.b as i64, instr.c),
            4 => OpCode::banr(instr.a, instr.b, instr.c),
            5 => OpCode::bani(instr.a, instr.b as i64, instr.c),
            6 => OpCode::borr(instr.a, instr.b, instr.c),
            7 => OpCode::bori(instr.a, instr.b as i64, instr.c),
            8 => OpCode::setr(instr.a, instr.b, instr.c),
            9 => OpCode::seti(instr.a as i64, instr.b, instr.c),
            10 => OpCode::gtir(instr.a as i64, instr.b, instr.c),
            11 => OpCode::gtri(instr.a, instr.b as i64, instr.c),
            12 => OpCode::gtrr(instr.a, instr.b, instr.c),
            13 => OpCode::eqir(instr.a as i64, instr.b, instr.c),
            14 => OpCode::eqri(instr.a, instr.b as i64, instr.c),
            15 => OpCode::eqrr(instr.a, instr.b, instr.c),
            _ => panic!("unknown operation position"),
        }
    }

    fn apply(&self, reg: &mut Registers) {
        match *self {
            OpCode::addr(a, b, c) => reg[c] = reg[a] + reg[b],
            OpCode::addi(a, b, c) => reg[c] = reg[a] + b,
            OpCode::mulr(a, b, c) => reg[c] = reg[a] * reg[b],
            OpCode::muli(a, b, c) => reg[c] = reg[a] * b,
            OpCode::banr(a, b, c) => reg[c] = reg[a] & reg[b],
            OpCode::bani(a, b, c) => reg[c] = reg[a] & b,
            OpCode::borr(a, b, c) => reg[c] = reg[a] | reg[b],
            OpCode::bori(a, b, c) => reg[c] = reg[a] | b,
            OpCode::setr(a, _, c) => reg[c] = reg[a],
            OpCode::seti(a, _, c) => reg[c] = a,
            OpCode::gtir(a, b, c) => reg[c] = if a > reg[b] { 1 } else { 0 },
            OpCode::gtri(a, b, c) => reg[c] = if reg[a] > b { 1 } else { 0 },
            OpCode::gtrr(a, b, c) => reg[c] = if reg[a] > reg[b] { 1 } else { 0 },
            OpCode::eqir(a, b, c) => reg[c] = if a == reg[b] { 1 } else { 0 },
            OpCode::eqri(a, b, c) => reg[c] = if reg[a] == b { 1 } else { 0 },
            OpCode::eqrr(a, b, c) => reg[c] = if reg[a] == reg[b] { 1 } else { 0 },
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, Default)]
struct Sample {
    before: Registers,
    instr: Instruction,
    after: Registers,
}

fn parse(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let mut parts = input.split("\n\n\n");
    let samples = parts.next().unwrap();
    let test_program = parts.next().unwrap();

    let samples = samples
        .split("\n\n")
        .map(|input| {
            let mut lines = input.lines();
            let mut sample = Sample::default();
            scan!(lines.next().unwrap().bytes() => "Before: [{}, {}, {}, {}]",
                sample.before[0], sample.before[1], sample.before[2], sample.before[3]);
            scan!(lines.next().unwrap().bytes() => "{} {} {} {}",
                sample.instr.opcode, sample.instr.a, sample.instr.b, sample.instr.c);
            scan!(lines.next().unwrap().bytes() => "After:  [{}, {}, {}, {}]",
            sample.after[0], sample.after[1], sample.after[2], sample.after[3]);
            sample
        })
        .collect();

    let instructions = test_program.lines().filter(|l| !l.trim().is_empty());
    let test_program = instructions
        .map(|l| {
            let mut instr = Instruction::default();
            scan!(l.bytes() => "{} {} {} {}", instr.opcode, instr.a, instr.b, instr.c);
            instr
        })
        .collect();

    (samples, test_program)
}

fn possible_opcodes(instr: &Instruction) -> [OpCode; 16] {
    [
        OpCode::addr(instr.a, instr.b, instr.c),
        OpCode::addi(instr.a, instr.b as i64, instr.c),
        OpCode::mulr(instr.a, instr.b, instr.c),
        OpCode::muli(instr.a, instr.b as i64, instr.c),
        OpCode::banr(instr.a, instr.b, instr.c),
        OpCode::bani(instr.a, instr.b as i64, instr.c),
        OpCode::borr(instr.a, instr.b, instr.c),
        OpCode::bori(instr.a, instr.b as i64, instr.c),
        OpCode::setr(instr.a, instr.b, instr.c),
        OpCode::seti(instr.a as i64, instr.b, instr.c),
        OpCode::gtir(instr.a as i64, instr.b, instr.c),
        OpCode::gtri(instr.a, instr.b as i64, instr.c),
        OpCode::gtrr(instr.a, instr.b, instr.c),
        OpCode::eqir(instr.a as i64, instr.b, instr.c),
        OpCode::eqri(instr.a, instr.b as i64, instr.c),
        OpCode::eqrr(instr.a, instr.b, instr.c),
    ]
}

fn part1(samples: &[Sample]) -> usize {
    samples
        .iter()
        .map(|sample| {
            let Sample {
                before,
                instr,
                after,
            } = sample;
            possible_opcodes(instr)
                .into_iter()
                .filter(|opcode| {
                    let mut regs = *before;
                    opcode.apply(&mut regs);
                    &regs == after
                })
                .count()
        })
        .filter(|&count| count >= 3)
        .count()
}

fn resolve_opcodes(samples: &[Sample]) -> [usize; 16] {
    let mut translation_table: [usize; 16 * 16] = [0; 16 * 16];
    for sample in samples {
        let possible_opcodes = possible_opcodes(&sample.instr);
        let applicable_opcodes = possible_opcodes
            .into_iter()
            .enumerate()
            .filter(|(_, opcode)| {
                let mut regs = sample.before;
                opcode.apply(&mut regs);
                regs == sample.after
            })
            .map(|(i, _)| i);
        for opcode in applicable_opcodes {
            translation_table[sample.instr.opcode * 16 + opcode] += 1;
        }
    }

    let mut res: [usize; 16] = [0; 16];
    let mut unresolved_opcodes: Vec<usize> = (0..16).collect();
    while !unresolved_opcodes.is_empty() {
        // find opcode with unique max usages
        let (opcode_pos, &opcode, mapped_opcode) = unresolved_opcodes
            .iter()
            .enumerate()
            .filter_map(|(opcode_pos, opcode)| {
                let translation = &translation_table[opcode * 16..(opcode + 1) * 16];
                // find max pos and value of this opcode
                let (max_pos, max_count) = translation
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, count)| *count)
                    .unwrap();
                // check if it is unique
                let is_unique = translation
                    .iter()
                    .filter(|count| *count == max_count)
                    .count()
                    == 1;
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
            translation_table[opcode * 16 + mapped_opcode] = 0;
        }
        // removed this opcode from unresolved opcodes
        unresolved_opcodes.swap_remove(opcode_pos);
    }

    let mut check = res;
    check.sort_unstable();
    assert_eq!(&check, &(0..16).collect::<Vec<_>>()[..]);

    res
}

fn part2(samples: &[Sample], test_program: &[Instruction]) -> i64 {
    let translation_table = resolve_opcodes(samples);
    let res = test_program
        .iter()
        .fold(Registers::default(), |mut reg, instr| {
            let op = OpCode::from_instr(instr, &translation_table);
            op.apply(&mut reg);
            reg
        });
    res[0]
}

pub fn solve(input: &str) -> (usize, i64) {
    let (samples, test_program) = parse(input);
    (part1(&samples), part2(&samples, &test_program))
}

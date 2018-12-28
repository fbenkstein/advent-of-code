use text_io::{scan, try_scan};

type Registers = [i64; 6];

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum Instruction {
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

impl Instruction {
    fn apply(&self, reg: &mut Registers) {
        match *self {
            Instruction::addr(a, b, c) => reg[c] = reg[a] + reg[b],
            Instruction::addi(a, b, c) => reg[c] = reg[a] + b,
            Instruction::mulr(a, b, c) => reg[c] = reg[a] * reg[b],
            Instruction::muli(a, b, c) => reg[c] = reg[a] * b,
            Instruction::banr(a, b, c) => reg[c] = reg[a] & reg[b],
            Instruction::bani(a, b, c) => reg[c] = reg[a] & b,
            Instruction::borr(a, b, c) => reg[c] = reg[a] | reg[b],
            Instruction::bori(a, b, c) => reg[c] = reg[a] | b,
            Instruction::setr(a, _, c) => reg[c] = reg[a],
            Instruction::seti(a, _, c) => reg[c] = a,
            Instruction::gtir(a, b, c) => reg[c] = if a > reg[b] { 1 } else { 0 },
            Instruction::gtri(a, b, c) => reg[c] = if reg[a] > b { 1 } else { 0 },
            Instruction::gtrr(a, b, c) => reg[c] = if reg[a] > reg[b] { 1 } else { 0 },
            Instruction::eqir(a, b, c) => reg[c] = if a == reg[b] { 1 } else { 0 },
            Instruction::eqri(a, b, c) => reg[c] = if reg[a] == b { 1 } else { 0 },
            Instruction::eqrr(a, b, c) => reg[c] = if reg[a] == reg[b] { 1 } else { 0 },
        }
    }
}

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    let ip: usize;
    scan!(lines.next().unwrap().bytes() => "#ip {}", ip);
    let instructions = lines
        .map(|line| {
            let op: String;
            let a: usize;
            let b: usize;
            let c: usize;
            scan!(line.bytes() => "{} {} {} {}", op, a, b, c);
            match &op[..] {
                "addr" => Instruction::addr(a, b, c),
                "addi" => Instruction::addi(a, b as i64, c),
                "mulr" => Instruction::mulr(a, b, c),
                "muli" => Instruction::muli(a, b as i64, c),
                "banr" => Instruction::banr(a, b, c),
                "bani" => Instruction::bani(a, b as i64, c),
                "borr" => Instruction::borr(a, b, c),
                "bori" => Instruction::bori(a, b as i64, c),
                "setr" => Instruction::setr(a, b, c),
                "seti" => Instruction::seti(a as i64, b, c),
                "gtir" => Instruction::gtir(a as i64, b, c),
                "gtri" => Instruction::gtri(a, b as i64, c),
                "gtrr" => Instruction::gtrr(a, b, c),
                "eqir" => Instruction::eqir(a as i64, b, c),
                "eqri" => Instruction::eqri(a, b as i64, c),
                "eqrr" => Instruction::eqrr(a, b, c),
                _ => panic!(),
            }
        })
        .collect();
    (ip, instructions)
}

fn exec(ip_reg: usize, program: &[Instruction], reg: &mut Registers) {
    let mut ip = 0;
    while let Some(instr) = program.get(ip) {
        reg[ip_reg] = ip as i64;
        instr.apply(reg);
        ip = reg[ip_reg] as usize;
        ip += 1;
    }
}

/// Sum of all divisor of n
///
/// https://en.wikipedia.org/wiki/Divisor_function
fn sigma_1(n: usize) -> usize {
    // The function is multiplicative when gcd of factors is 1, so we can just calculate it on the
    // prime divisors.
    let sieve = primal::Sieve::new((n as f64).sqrt().floor() as usize);
    let divisors = sieve.factor(n).expect("wrong sieve upper bound");
    divisors
        .into_iter()
        // sigma_1(p^n) = p^(n+1) / (p - 1) by geometry sum, and
        // sigma_1(p) = p + 1, since p does not have other divisors
        .map(|(p, exp)| (p.pow(exp as u32 + 1) - 1) / (p - 1))
        .product()
}

pub fn solve(input: &str) -> (i64, usize) {
    let (ip, program) = parse(input);

    let mut reg1 = [0, 0, 0, 0, 0, 0];
    exec(ip, &program, &mut reg1);

    // stupid reverse engineering mess shows that we are actually computing the sum of prime factors :/
    let a = match program[21] {
        Instruction::addi(_, b, _) => b,
        _ => panic!("reverse engineering was wrong"),
    };
    let b = match program[23] {
        Instruction::addi(_, b, _) => b,
        _ => panic!("reverse engineering was wrong"),
    };
    let n = 10551236 + a * 22 + b;
    (reg1[0], sigma_1(n as usize))
}

mod parser;

use crate::parser::*;

use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let (ip_reg, instructions) = parser::parse_instructions(CompleteStr(&contents))
        .expect("Could not parse?")
        .1;

    let mut registers: Registers = [0; 6];
    run(ip_reg, &instructions, &mut registers);
    println!("Registers after execution of program: {:?}", registers);
}

fn run(ip_reg: usize, instructions: &Vec<Instruction>, registers: &mut Registers) {
    let mut ip = 0;
    let mut seen = BTreeSet::new();
    let mut last_seen = 0;
    let mut last = 0;

    while ip < instructions.len() {
        if registers[ip_reg] == 28 {
            // println!("Lowest non-negative integer value for register 0 that causes the program to halt after executing the fewest instructions: {}");
            let value = registers[instructions[28].a];
            if !seen.contains(&value) {
                last_seen = value;
                seen.insert(value);
            } else {
                println!("Lowest non-negative integer value for register 0 that causes the program to halt after executing the most instructions: {}", last_seen);
                return;
            }
        }

        registers[ip_reg] = ip;

        if let Some(instruction) = &instructions.get(ip) {
            instruction.execute(registers);
            ip = registers[ip_reg] + 1;
        } else {
            return;
        }
    }
}

#[test]
fn test_device() {
    let input = r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#;

    let (ip, instructions) = parser::parse_instructions(CompleteStr(&input))
        .expect("Parsing error")
        .1;
    assert_eq!(ip, 0);
    assert_eq!(instructions.len(), 7);
}

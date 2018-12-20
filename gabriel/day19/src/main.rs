mod parser;

use crate::parser::*;

use std::fs::File;
use std::io::prelude::*;

use log::debug;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let (ip_reg, instructions) = parser::parse_instructions(CompleteStr(&contents))
        .expect("Could not parse?")
        .1;

    println!(
        "Registers after execution of program: {:?}",
        run(ip_reg, instructions)
    );
}

fn run(ip_reg: usize, instructions: Vec<Instruction>) -> Registers {
    let mut instruction_pointer = 0;
    let mut registers: Registers = [0; 6];

    loop {
        registers[ip_reg] = instruction_pointer;

        if let Some(instruction) = &instructions.get(instruction_pointer) {
            debug!("ip={} {:?} ", instruction_pointer, registers);
            instruction.execute(&mut registers);
            instruction_pointer = registers[ip_reg];
            instruction_pointer += 1;
            debug!("{} {:?}", instruction, registers);
        } else {
            return registers;
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

mod parser;

use crate::parser::*;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file");

    let (ip_reg, mut instructions) = parser::parse_instructions(CompleteStr(&contents))
        .expect("Could not parse?")
        .1;

    let mut registers: Registers = [0; 6];
    run(ip_reg, &instructions, &mut registers);
    println!("Registers after execution of program: {:?}", registers);

    let mut registers: Registers = [1, 0, 0, 0, 0, 0];
    run(ip_reg, &instructions, &mut registers);
    println!("Registers after execution of program: {:?}", registers);
    let p2 = registers[2];
    println!(
        "The program is trying to do: {}",
        p2 + (1..=p2 / 2).filter(|x| p2 % x == 0).sum::<usize>()
    );
}

fn run(ip_reg: usize, instructions: &Vec<Instruction>, registers: &mut Registers) {
    let mut instruction_pointer = 0;
    let mut halt_counter = 0;

    while instruction_pointer < instructions.len() {
        registers[ip_reg] = instruction_pointer;

        if let Some(instruction) = &instructions.get(instruction_pointer) {
            instruction.execute(registers);
            instruction_pointer = registers[ip_reg];
            if instruction_pointer == 1 {
                print!("ip={:<2} {:<4?} ", instruction_pointer, registers);
                println!("{} {:<4?}", instruction, registers);
                halt_counter += 1;
                if halt_counter > 5 {
                    println!("OS HALTED EXECUTION");
                    return;
                }
            }
            instruction_pointer += 1;
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

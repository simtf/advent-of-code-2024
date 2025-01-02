use itertools::Itertools;
use std::fs;
use std::ops::BitXor;

struct Register {
    a: u32,
    b: u32,
    c: u32,
}

const VERBOSE: bool = true;

const TEST: bool = false;

const PATH: &str = if TEST {
    "./resources/day_17_example.txt"
} else {
    "./resources/day_17.txt"
};

pub fn execute() {
    let dataset: String = fs::read_to_string(PATH).unwrap();
    let lines: Vec<&str> = dataset.split("\n\n").collect();

    let registers: Vec<&str> = lines[0].split("\n").collect();
    let mut register = Register {
        a: registers[0]
            .strip_prefix("Register A: ")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        b: registers[1]
            .strip_prefix("Register B: ")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
        c: registers[2]
            .strip_prefix("Register C: ")
            .unwrap()
            .parse::<u32>()
            .unwrap(),
    };

    let program: Vec<u32> = lines[1]
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|p| p.parse::<u32>().unwrap())
        .collect();

    if VERBOSE {
        println!("Initial state:");
        println!(" - Register A: {}", register.a);
        println!(" - Register B: {}", register.b);
        println!(" - Register C: {}", register.c);
        println!();
        println!("Execute program: {:?}", program);
        println!();
    }

    let out: String = compute_program(&program, &mut register)
        .iter()
        .map(|d| d.to_string())
        .join(",");

    if VERBOSE {
        println!("Final state:");
        println!(" - Register A: {}", register.a);
        println!(" - Register B: {}", register.b);
        println!(" - Register C: {}", register.c);
        println!();
    }

    println!("Out: {out}");
}

fn compute_program(program: &Vec<u32>, register: &mut Register) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    while i < program.len() - 1 {
        let opcode: u32 = program[i];
        let operand: u32 = program[i + 1];
        match opcode {
            0 => {
                register.a = register.a.div_euclid(2_u32.pow(combo(operand, &register)));
                i += 2;
            }
            1 => {
                register.b = register.b.bitxor(operand);
                i += 2;
            }
            2 => {
                register.b = combo(operand, &register) % 8;
                i += 2;
            }
            3 => {
                i = if register.a > 0 {
                    usize::try_from(operand).unwrap()
                } else {
                    i + 2
                };
            }
            4 => {
                register.b = register.b.bitxor(register.c);
                i += 2;
            }
            5 => {
                out.push(combo(operand, &register) % 8);
                i += 2;
            }
            6 => {
                register.b = register.a.div_euclid(2_u32.pow(combo(operand, &register)));
                i += 2;
            }
            7 => {
                register.c = register.a.div_euclid(2_u32.pow(combo(operand, &register)));
                i += 2;
            }
            _ => {}
        }
    }
    out
}

fn combo(operand: u32, register: &Register) -> u32 {
    match operand {
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => operand,
    }
}

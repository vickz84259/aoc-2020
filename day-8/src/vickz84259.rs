use std::fs::File;
use std::io::{self, BufRead};
use std::mem;
use std::str::FromStr;
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug)]
struct CPU {
    accumulator: usize,
    ip: usize,
    instructions: Vec<String>,
    history: Vec<usize>,
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Sub(usize),
}
use Operation::{Add, Sub};

#[derive(Debug)]
enum CPUResult {
    InfiniteLoop,
    Terminate,
}
use CPUResult::{InfiniteLoop, Terminate};

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Operation, Self::Err> {
        let value: usize = match s[1..].parse() {
            Ok(value) => value,
            Err(_) => return Err("Argument does not contain a valid number"),
        };

        match s.chars().nth(0) {
            Some('+') => Ok(Operation::Add(value)),
            Some('-') => Ok(Operation::Sub(value)),
            _ => Err("Argument contains an invalid sign"),
        }
    }
}

impl CPU {
    fn new() -> Self {
        CPU {
            accumulator: 0,
            ip: 0,
            instructions: Vec::with_capacity(654),
            history: Vec::with_capacity(50),
        }
    }

    fn nop(&mut self) {
        self.ip += 1;
    }

    fn acc(&mut self, operation: Operation) {
        match operation {
            Add(value) => self.accumulator += value,
            Sub(value) => self.accumulator -= value,
        };
        self.ip += 1;
    }

    fn jmp(&mut self, operation: Operation) {
        match operation {
            Add(value) => self.ip += value,
            Sub(value) => self.ip -= value,
        };
    }

    fn get_instruction(&self, address: &usize) -> (&str, &str) {
        self.instructions[*address]
            .split_whitespace()
            .collect_tuple()
            .unwrap()
    }

    fn swap_instruction(&mut self, address: &usize) {
        let (new_instruction, arg_str) = match self.get_instruction(address) {
            ("nop", arg) => ("jmp", arg),
            ("jmp", arg) => ("nop", arg),
            _ => return (),
        };

        let mut updated_instruction =
            String::with_capacity(new_instruction.len() + arg_str.len() + 1);
        updated_instruction.push_str(new_instruction);
        updated_instruction.push(' ');
        updated_instruction.push_str(arg_str);

        let _ = mem::replace(&mut self.instructions[*address], updated_instruction);
    }

    fn step(&mut self) {
        self.history.push(self.ip);

        let (instruction, arg_str) = self.get_instruction(&self.ip);
        let operation: Operation = arg_str.parse().unwrap();

        match instruction {
            "acc" => self.acc(operation),
            "jmp" => self.jmp(operation),
            "nop" => self.nop(),
            _ => panic!("Invalid Instruction"),
        };
    }

    fn run(&mut self) -> CPUResult {
        loop {
            if self.history.contains(&self.ip) {
                return InfiniteLoop;
            }

            if self.ip >= self.instructions.len() {
                return Terminate;
            }
            self.step();
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.accumulator = 0;
        self.history.clear();
    }
}

fn get_cpu() -> CPU {
    let file = File::open("../input.txt").expect("Unable to open file");

    let mut cpu = CPU::new();
    io::BufReader::new(file)
        .lines()
        .for_each(|line| cpu.instructions.push(line.unwrap()));

    cpu
}

fn part_1(cpu: &mut CPU) {
    match cpu.run() {
        InfiniteLoop => {
            println!("Infinite loop detected at address: {}", cpu.ip);
            println!("Accumulator value: {}", cpu.accumulator);
        }
        Terminate => (),
    }
}

fn part_2(cpu: &mut CPU) {
    // First run. It should have an infinite loop
    match cpu.run() {
        InfiniteLoop => (),
        Terminate => panic!("CPU terminated when it shouldn't."),
    }

    let history = cpu.history.clone();
    for address in history {
        cpu.reset();

        let (old_instruction, _) = cpu.get_instruction(&address);
        match old_instruction {
            "nop" | "jmp" => cpu.swap_instruction(&address),
            _ => continue,
        };

        match cpu.run() {
            InfiniteLoop => cpu.swap_instruction(&address),
            Terminate => {
                println!("Accumulator value: {}", cpu.accumulator);
                break;
            }
        };
    }
}

fn main() {
    println!("Part 1: \n----------");
    let mut cpu = get_cpu();

    let mut start = Instant::now();
    part_1(&mut cpu);
    println!("Time Taken: {:?}", start.elapsed());

    println!("----------");
    println!("Part 2: \n----------");

    start = Instant::now();
    part_2(&mut cpu);
    println!("Time Taken: {:?}", start.elapsed());
}

/* Advent of Code 2020, https://adventofcode.com/2020/day/8 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("08.test");
    assert_eq!(acc_at_loop(&input), 5);

    let input = read_input("08.input");
    println!("Part 1: {}", acc_at_loop(&input));
}

fn acc_at_loop(input: &Input) -> i32 {
    let (acc, term) = acc_at_loop_or_term(input);
    assert!(!term);
    acc
}

fn acc_at_loop_or_term(input: &Input) -> (i32, bool) {
    let mut visited =
        input
        .program
        .iter()
        .map(|_| false)
        .collect::<Vec<bool>>();
    let mut ip = 0;
    let mut acc = 0;
    loop {
        if ip >= input.program.len() {
            assert_eq!(ip, input.program.len());
            return (acc, true);
        } else if visited[ip] {
            return (acc, false);
        }
        visited[ip] = true;
        match input.program[ip] {
            Instruction::Acc { i } => {
                acc += i;
                ip += 1;
            }
            Instruction::Jmp { i } => {
                ip = (ip as i32 + i) as usize;
            }
            Instruction::Nop { i: _ } => {
                ip += 1;
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Acc { i: i32 },
    Jmp { i: i32 },
    Nop { i: i32 },
}

#[derive(Debug)]
struct Input {
    program: Vec<Instruction>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Input {
        program: input_file
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut split = line.split_whitespace();
                let name = split.next().unwrap().to_string();
                let val = i32::from_str(split.next().unwrap()).unwrap();
                match name.as_str() {
                    "acc" => Instruction::Acc { i: val },
                    "jmp" => Instruction::Jmp { i: val },
                    "nop" => Instruction::Nop { i: val },
                    _ => panic!("Unknown instruction {}", name),
                }
            })
            .collect::<Vec<Instruction>>()
    }
}

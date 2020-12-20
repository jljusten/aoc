/* Advent of Code 2020, https://adventofcode.com/2020/day/14 */

/* Jordan Justen : this file is public domain */

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("14.test");
    assert_eq!(part1(&input), 165);

    let input = read_input("14.test2");
    assert_eq!(part2(&input), 208);

    let input = read_input("14.input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = 0u64;
    let mut mask_val = 0u64;
    for inst in input.inst.iter() {
        match inst {
            Instruction::Mask { mask: imask, val } => {
                mask = *imask;
                mask_val = *val;
            }
            Instruction::Mem { addr, val } => {
                let entry = mem.entry(*addr).or_default();
                *entry = (val & mask) | mask_val;
            }
        }
    }
    mem.values().fold(0, |a, &x| a + x)
}

fn part2(input: &Input) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = 0u64;
    let mut mask_val = 0u64;
    for inst in input.inst.iter() {
        match inst {
            Instruction::Mask { mask: imask, val } => {
                mask = *imask;
                mask_val = *val;
            }
            Instruction::Mem { addr, val } => {
                let set_addr = (addr & !mask) | mask_val;
                let floating_bits = (0usize..36)
                    .filter(|b| (mask & (1u64 << b)) != 0)
                    .collect::<Vec<usize>>();
                for v in 0..(1 << floating_bits.len()) {
                    let mut addr = set_addr;
                    for b in floating_bits.iter().enumerate() {
                        let or_in = (v & (1 << b.0)) << (b.1 - b.0);
                        addr = addr | or_in;
                    }
                    let entry = mem.entry(addr).or_default();
                    *entry = *val;
                }
            }
        }
    }
    mem.values().fold(0, |a, &x| a + x)
}

#[derive(Debug)]
enum Instruction {
    Mask { mask: u64, val: u64 },
    Mem { addr: u64, val: u64 },
}

#[derive(Debug)]
struct Input {
    inst: Vec<Instruction>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    let lines = input_file.lines();
    let inst: Vec<Instruction> =
        lines
            .map(|ln| {
                let ln = ln.unwrap();
                if ln.starts_with("mask = ") {
                    let (mask, val) = ln.chars().skip(7).collect::<Vec<char>>().iter().fold(
                        (0u64, 0u64),
                        |a, c| {
                            let (mask, val) = match c {
                                'X' => (1, 0),
                                '1' => (0, 1),
                                '0' => (0, 0),
                                _ => panic!("Bad mask char: {}", c),
                            };
                            ((a.0 << 1) | mask, (a.1 << 1) | val)
                        },
                    );
                    assert!(mask < (1u64 << 36) && val < (1u64 << 36));
                    Instruction::Mask { mask, val }
                } else {
                    assert!(ln.starts_with("mem["));
                    let close = ln.find("]").unwrap();
                    let addr = u64::from_str(
                        ln.chars()
                            .skip(4)
                            .take(close - 4)
                            .collect::<String>()
                            .as_str(),
                    )
                    .unwrap();
                    let val_str = ln.split(" = ").skip(1).next().unwrap();
                    let val = u64::from_str(val_str).unwrap();
                    Instruction::Mem { addr, val }
                }
            })
            .collect();
    Input { inst }
}

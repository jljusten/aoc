/* Advent of Code 2020, https://adventofcode.com/2020/day/12 */

/* Jordan Justen : this file is public domain */

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn main() {
    let input = read_input("12.test");
    assert_eq!(part1(&input), 25);

    let input = read_input("12.input");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    let pos = input
        .inst
        .iter()
        .scan((Direction::E, 0isize, 0isize), |s, i| {
            match *i {
                Instruction::Directed { d: Direction::N, n } => s.2 += n as isize,
                Instruction::Directed { d: Direction::S, n } => s.2 -= n as isize,
                Instruction::Directed { d: Direction::E, n } => s.1 += n as isize,
                Instruction::Directed { d: Direction::W, n } => s.1 -= n as isize,
                Instruction::Turn { t: Turn::L, d } => {
                    s.0 = n_to_dir(dir_to_n(s.0) + 360 - d);
                }
                Instruction::Turn { t: Turn::R, d } => {
                    s.0 = n_to_dir(dir_to_n(s.0) + d);
                }
                Instruction::F { n } => {
                    match s.0 {
                        Direction::N => s.2 += n as isize,
                        Direction::S => s.2 -= n as isize,
                        Direction::E => s.1 += n as isize,
                        Direction::W => s.1 -= n as isize,
                    }
                }
            }
            Some((s.1, s.2))
        })
        .last()
        .unwrap();
    (pos.0.abs() + pos.1.abs()) as usize
}

fn dir_to_n(d: Direction) -> usize {
    match d {
        Direction::N => 0,
        Direction::S => 180,
        Direction::E => 90,
        Direction::W => 270,
    }
}

fn n_to_dir(d: usize) -> Direction {
    match d % 360 {
        0 => Direction::N,
        180 => Direction::S,
        90 => Direction::E,
        270 => Direction::W,
        _ => panic!("Unsupported direction number"),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, PartialEq)]
enum Turn {
    L,
    R,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Directed { d: Direction, n: usize },
    Turn { t: Turn, d: usize },
    F { n: usize },
}

#[derive(Debug)]
struct Input {
    inst: Vec<Instruction>,
}

fn read_input(fname: &str) -> Input {
    let input_f = File::open(fname).unwrap();
    let input_file = BufReader::new(&input_f);
    Input {
        inst: input_file
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (ty, num) = line.split_at(1);
                let ty = ty.chars().next().unwrap();
                let n = usize::from_str(num).unwrap();
                match ty {
                    'N' => Instruction::Directed { d: Direction::N, n },
                    'S' => Instruction::Directed { d: Direction::S, n },
                    'E' => Instruction::Directed { d: Direction::E, n },
                    'W' => Instruction::Directed { d: Direction::W, n },
                    'L' => Instruction::Turn { t: Turn::L, d: n },
                    'R' => Instruction::Turn { t: Turn::R, d: n },
                    'F' => Instruction::F { n },
                    _ => panic!("Unsupported char: {}", ty),
                }
            })
            .collect(),
    }
}
